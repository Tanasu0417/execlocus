use std::env;

use crate::model::{
    Confidence, Evidence, ObservationStatus, PathClass, ProbeFailure, ProbeResult, ProjectInfo,
    RuntimeKind,
};

#[must_use]
pub fn probe_project(runtime: &RuntimeKind) -> ProbeResult<ProjectInfo> {
    match env::current_dir() {
        Ok(path) => {
            let path_text = path.to_string_lossy().into_owned();
            let class = classify_path(&path_text, *runtime);
            ProbeResult {
                value: ProjectInfo {
                    path: Some(path_text.clone()),
                    class,
                    status: ObservationStatus::Observed,
                    confidence: Confidence::Certain,
                },
                evidence: vec![Evidence {
                    id: "project.path".to_owned(),
                    probe: "path/v1".to_owned(),
                    kind: "filesystem".to_owned(),
                    claim: format!("project path classified as {class:?}"),
                    value: Some(path_text),
                    sensitive: true,
                }],
                failures: Vec::new(),
            }
        }
        Err(error) => ProbeResult {
            value: ProjectInfo {
                path: None,
                class: PathClass::Unknown,
                status: ObservationStatus::Failed,
                confidence: Confidence::None,
            },
            evidence: Vec::new(),
            failures: vec![ProbeFailure {
                probe: "path/v1".to_owned(),
                code: "CURRENT_DIR_UNAVAILABLE".to_owned(),
                message: error.to_string(),
            }],
        },
    }
}

#[must_use]
pub fn classify_path(path: &str, runtime: RuntimeKind) -> PathClass {
    let normalized = path.replace('\\', "/");
    let lower = normalized.to_ascii_lowercase();

    if lower.starts_with("//wsl.localhost/") || lower.starts_with("//wsl$/") {
        return PathClass::WslUnc;
    }

    if is_windows_drive_path(path) {
        return PathClass::WindowsNative;
    }

    if is_wsl_mounted_drive(&lower) {
        return PathClass::WindowsMounted;
    }

    if normalized.starts_with('/') {
        return match runtime {
            RuntimeKind::Wsl => PathClass::WslNative,
            RuntimeKind::LinuxNative => PathClass::LinuxNative,
            _ => PathClass::Unknown,
        };
    }

    PathClass::Unknown
}

fn is_windows_drive_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'\\' | b'/')
}

fn is_wsl_mounted_drive(lower: &str) -> bool {
    let bytes = lower.as_bytes();
    bytes.len() >= 7
        && bytes.starts_with(b"/mnt/")
        && bytes[5].is_ascii_lowercase()
        && bytes[6] == b'/'
}

#[cfg(test)]
mod tests {
    use super::classify_path;
    use crate::model::{PathClass, RuntimeKind};

    #[test]
    fn classifies_windows_drive() {
        assert_eq!(
            classify_path(r"C:\Users\dev\project", RuntimeKind::WindowsNative),
            PathClass::WindowsNative
        );
    }

    #[test]
    fn classifies_wsl_mounted_drive() {
        assert_eq!(
            classify_path("/mnt/c/Users/dev/project", RuntimeKind::Wsl),
            PathClass::WindowsMounted
        );
    }

    #[test]
    fn classifies_wsl_native_path() {
        assert_eq!(
            classify_path("/home/dev/project", RuntimeKind::Wsl),
            PathClass::WslNative
        );
    }

    #[test]
    fn classifies_linux_native_path_independently_of_host_os() {
        assert_eq!(
            classify_path("/srv/project", RuntimeKind::LinuxNative),
            PathClass::LinuxNative
        );
    }

    #[test]
    fn classifies_wsl_unc_path() {
        assert_eq!(
            classify_path(
                r"\\wsl.localhost\Ubuntu-24.04\home\dev\project",
                RuntimeKind::WindowsNative
            ),
            PathClass::WslUnc
        );
    }
}
