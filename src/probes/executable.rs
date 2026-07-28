use std::{
    collections::HashSet,
    env, fs,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    model::{
        Confidence, Evidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo,
        ExecutableOrigin, ObservationStatus, ProbeResult, RuntimeKind,
    },
    probes::path::classify_path,
};

#[must_use]
pub fn probe(role: &str, command: &str, runtime: &RuntimeKind) -> ProbeResult<ExecutableInfo> {
    let candidates = resolve_candidates(command, *runtime);
    let selected = candidates.first().cloned();
    let status = if selected.is_some() {
        ObservationStatus::Observed
    } else {
        ObservationStatus::Unavailable
    };
    let confidence = if selected.is_some() {
        Confidence::Certain
    } else {
        Confidence::None
    };

    let evidence = selected.as_ref().map_or_else(Vec::new, |candidate| {
        vec![Evidence {
            id: format!("executable.{role}"),
            probe: "executable/v1".to_owned(),
            kind: "executable".to_owned(),
            claim: format!("{role} resolves to {:?} executable", candidate.origin),
            value: Some(candidate.path.clone()),
            sensitive: true,
        }]
    });

    ProbeResult {
        value: ExecutableInfo {
            role: role.to_owned(),
            requested: command.to_owned(),
            selected,
            candidates,
            status,
            confidence,
        },
        evidence,
        failures: Vec::new(),
    }
}

#[must_use]
pub fn resolve_candidates(command: &str, runtime: RuntimeKind) -> Vec<ExecutableCandidate> {
    let Some(path_value) = env::var_os("PATH") else {
        return Vec::new();
    };

    let names = candidate_names(command, runtime);
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();

    for directory in env::split_paths(&path_value) {
        for name in &names {
            let candidate_path = directory.join(name);
            if !candidate_path.is_file() || !is_executable(&candidate_path) {
                continue;
            }

            let inspected = inspect_candidate(&candidate_path, runtime);
            let key = if runtime == RuntimeKind::WindowsNative {
                inspected.path.to_ascii_lowercase()
            } else {
                inspected.path.clone()
            };
            if !seen.insert(key) {
                continue;
            }

            candidates.push(inspected);
        }
    }

    candidates
}

fn candidate_names(command: &str, runtime: RuntimeKind) -> Vec<String> {
    if runtime != RuntimeKind::WindowsNative || Path::new(command).extension().is_some() {
        return vec![command.to_owned()];
    }

    let extensions = env::var("PATHEXT")
        .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_owned())
        .split(';')
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();

    let mut names = vec![command.to_owned()];
    names.extend(
        extensions
            .into_iter()
            .map(|extension| format!("{command}{extension}")),
    );
    names
}

fn inspect_candidate(path: &Path, runtime: RuntimeKind) -> ExecutableCandidate {
    let resolved = fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
    let format = detect_format(&resolved);
    let path_text = display_path(&resolved);
    let origin = match format {
        ExecutableFormat::Pe => ExecutableOrigin::Windows,
        ExecutableFormat::Elf => ExecutableOrigin::Linux,
        ExecutableFormat::Script => script_origin(&resolved),
        ExecutableFormat::Unknown => {
            let class = classify_path(&path_text, runtime);
            if matches!(
                class,
                crate::model::PathClass::WindowsNative | crate::model::PathClass::WindowsMounted
            ) && matches!(
                resolved
                    .extension()
                    .and_then(|value| value.to_str())
                    .map(str::to_ascii_lowercase)
                    .as_deref(),
                Some("exe" | "com" | "cmd" | "bat")
            ) {
                ExecutableOrigin::Windows
            } else {
                ExecutableOrigin::Unknown
            }
        }
    };

    ExecutableCandidate {
        path: path_text,
        format,
        origin,
    }
}

fn display_path(path: &Path) -> String {
    let text = path.to_string_lossy();
    if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{rest}")
    } else if let Some(rest) = text.strip_prefix(r"\\?\") {
        rest.to_owned()
    } else {
        text.into_owned()
    }
}

fn detect_format(path: &Path) -> ExecutableFormat {
    let Ok(mut file) = fs::File::open(path) else {
        return ExecutableFormat::Unknown;
    };
    let mut bytes = [0_u8; 4];
    let Ok(read) = file.read(&mut bytes) else {
        return ExecutableFormat::Unknown;
    };

    if read >= 2 && bytes.starts_with(b"MZ") {
        ExecutableFormat::Pe
    } else if read == 4 && bytes == [0x7f, b'E', b'L', b'F'] {
        ExecutableFormat::Elf
    } else if read >= 2 && bytes.starts_with(b"#!") {
        ExecutableFormat::Script
    } else {
        ExecutableFormat::Unknown
    }
}

fn script_origin(path: &Path) -> ExecutableOrigin {
    let Ok(contents) = fs::read_to_string(path) else {
        return ExecutableOrigin::Script;
    };
    let first_line = contents
        .lines()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if first_line.contains("/bin/") || first_line.contains("/usr/bin/") {
        ExecutableOrigin::Linux
    } else if first_line.contains("powershell") || first_line.contains("cmd.exe") {
        ExecutableOrigin::Windows
    } else {
        ExecutableOrigin::Script
    }
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
fn is_executable(_path: &Path) -> bool {
    true
}

#[cfg(not(any(unix, windows)))]
fn is_executable(_path: &Path) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use std::{env, fs, time::SystemTime};

    use super::{detect_format, display_path};
    use crate::model::ExecutableFormat;

    #[test]
    fn identifies_pe_and_elf_headers() {
        let suffix = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos();
        let base = env::temp_dir();
        let pe = base.join(format!("execlocus-{suffix}-pe.bin"));
        let elf = base.join(format!("execlocus-{suffix}-elf.bin"));

        fs::write(&pe, b"MZ\0\0").expect("write PE fixture");
        fs::write(&elf, [0x7f, b'E', b'L', b'F']).expect("write ELF fixture");

        assert_eq!(detect_format(&pe), ExecutableFormat::Pe);
        assert_eq!(detect_format(&elf), ExecutableFormat::Elf);

        fs::remove_file(pe).expect("remove PE fixture");
        fs::remove_file(elf).expect("remove ELF fixture");
    }

    #[test]
    fn hides_windows_verbatim_path_prefixes() {
        assert_eq!(
            display_path(std::path::Path::new(r"\\?\C:\Program Files\Git\git.exe")),
            r"C:\Program Files\Git\git.exe"
        );
        assert_eq!(
            display_path(std::path::Path::new(r"\\?\UNC\server\share\tool.exe")),
            r"\\server\share\tool.exe"
        );
    }
}
