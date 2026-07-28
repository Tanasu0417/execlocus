use std::{collections::HashSet, path::Path};

use crate::{
    model::{
        Confidence, Evidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo,
        ExecutableOrigin, ObservationStatus, ProbeFailure, ProbeResult, RuntimeKind,
    },
    probes::context::{CandidateSnapshot, ProbeContext, SystemProbeContext},
    probes::path::classify_path,
};

const EXECUTABLE_PREFIX_LIMIT: usize = 512;

pub trait ExecutableResolver {
    fn resolve(
        &self,
        context: &dyn ProbeContext,
        command: &str,
        runtime: RuntimeKind,
    ) -> ProbeResult<Vec<ExecutableCandidate>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PathExecutableResolver;

impl ExecutableResolver for PathExecutableResolver {
    fn resolve(
        &self,
        context: &dyn ProbeContext,
        command: &str,
        runtime: RuntimeKind,
    ) -> ProbeResult<Vec<ExecutableCandidate>> {
        resolve_from_path(context, command, runtime)
    }
}

#[must_use]
pub fn probe(role: &str, command: &str, runtime: &RuntimeKind) -> ProbeResult<ExecutableInfo> {
    probe_with(&SystemProbeContext, role, command, runtime)
}

#[must_use]
pub fn probe_with(
    context: &dyn ProbeContext,
    role: &str,
    command: &str,
    runtime: &RuntimeKind,
) -> ProbeResult<ExecutableInfo> {
    probe_with_resolver(context, &PathExecutableResolver, role, command, runtime)
}

#[must_use]
pub fn probe_with_resolver(
    context: &dyn ProbeContext,
    resolver: &dyn ExecutableResolver,
    role: &str,
    command: &str,
    runtime: &RuntimeKind,
) -> ProbeResult<ExecutableInfo> {
    let ProbeResult {
        value: candidates,
        mut evidence,
        failures,
    } = resolver.resolve(context, command, *runtime);
    let selected = candidates.first().cloned();
    let status = match (selected.is_some(), failures.is_empty()) {
        (true, _) => ObservationStatus::Observed,
        (false, true) => ObservationStatus::Unavailable,
        (false, false) => ObservationStatus::Failed,
    };
    let confidence = match (selected.is_some(), failures.is_empty()) {
        (true, true) => Confidence::Certain,
        (true, false) => Confidence::High,
        (false, _) => Confidence::None,
    };

    if let Some(candidate) = &selected {
        evidence.push(Evidence {
            id: format!("executable.{role}"),
            probe: "executable/v1".to_owned(),
            kind: "executable".to_owned(),
            claim: format!("{role} resolves to {:?} executable", candidate.origin),
            value: Some(candidate.path.clone()),
            sensitive: true,
        });
    }

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
        failures,
    }
}

#[must_use]
pub fn resolve_candidates(command: &str, runtime: RuntimeKind) -> Vec<ExecutableCandidate> {
    resolve_candidates_with(&SystemProbeContext, command, runtime).value
}

#[must_use]
pub fn resolve_candidates_with(
    context: &dyn ProbeContext,
    command: &str,
    runtime: RuntimeKind,
) -> ProbeResult<Vec<ExecutableCandidate>> {
    PathExecutableResolver.resolve(context, command, runtime)
}

fn resolve_from_path(
    context: &dyn ProbeContext,
    command: &str,
    runtime: RuntimeKind,
) -> ProbeResult<Vec<ExecutableCandidate>> {
    let names = candidate_names(context, command, runtime);
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    let mut failures = Vec::new();

    for directory in context.path_entries() {
        for name in &names {
            let snapshot =
                match context.inspect_candidate(&directory, name, EXECUTABLE_PREFIX_LIMIT) {
                    Ok(Some(snapshot)) => snapshot,
                    Ok(None) => continue,
                    Err(error) => {
                        failures.push(ProbeFailure {
                            probe: "executable/v1".to_owned(),
                            code: "CANDIDATE_INSPECTION_FAILED".to_owned(),
                            message: format!("failed to inspect a {command} candidate: {error}"),
                        });
                        continue;
                    }
                };
            if !snapshot.executable {
                continue;
            }

            let inspected = inspect_candidate(&snapshot, runtime);
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

    ProbeResult {
        value: candidates,
        evidence: Vec::new(),
        failures,
    }
}

fn candidate_names(context: &dyn ProbeContext, command: &str, runtime: RuntimeKind) -> Vec<String> {
    if runtime != RuntimeKind::WindowsNative || command_has_extension(command, runtime) {
        return vec![command.to_owned()];
    }

    let extensions = context
        .env_var("PATHEXT")
        .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".to_owned())
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

fn command_has_extension(command: &str, runtime: RuntimeKind) -> bool {
    if runtime == RuntimeKind::WindowsNative {
        let file_name = command.rsplit(['\\', '/']).next().unwrap_or(command);
        file_name
            .rsplit_once('.')
            .is_some_and(|(stem, extension)| !stem.is_empty() && !extension.is_empty())
    } else {
        Path::new(command).extension().is_some()
    }
}

fn inspect_candidate(snapshot: &CandidateSnapshot, runtime: RuntimeKind) -> ExecutableCandidate {
    let format = detect_format_from_prefix(&snapshot.prefix);
    let path_text = display_path(&snapshot.resolved_path);
    let origin = match format {
        ExecutableFormat::Pe => ExecutableOrigin::Windows,
        ExecutableFormat::Elf => ExecutableOrigin::Linux,
        ExecutableFormat::Script => script_origin(&snapshot.prefix),
        ExecutableFormat::Unknown => {
            let class = classify_path(&path_text, runtime);
            if matches!(
                class,
                crate::model::PathClass::WindowsNative | crate::model::PathClass::WindowsMounted
            ) && matches!(
                snapshot
                    .resolved_path
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

fn detect_format_from_prefix(bytes: &[u8]) -> ExecutableFormat {
    if bytes.starts_with(b"MZ") {
        ExecutableFormat::Pe
    } else if bytes.starts_with(b"\x7fELF") {
        ExecutableFormat::Elf
    } else if bytes.starts_with(b"#!") {
        ExecutableFormat::Script
    } else {
        ExecutableFormat::Unknown
    }
}

fn script_origin(prefix: &[u8]) -> ExecutableOrigin {
    let line_end = prefix
        .iter()
        .position(|byte| *byte == b'\n')
        .unwrap_or(prefix.len());
    let first_line = String::from_utf8_lossy(&prefix[..line_end]).to_ascii_lowercase();
    if first_line.contains("/bin/") || first_line.contains("/usr/bin/") {
        ExecutableOrigin::Linux
    } else if first_line.contains("powershell") || first_line.contains("cmd.exe") {
        ExecutableOrigin::Windows
    } else {
        ExecutableOrigin::Script
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_format_from_prefix, display_path, script_origin};
    use crate::model::{ExecutableFormat, ExecutableOrigin};

    #[test]
    fn identifies_executable_headers() {
        assert_eq!(detect_format_from_prefix(b"MZ\0\0"), ExecutableFormat::Pe);
        assert_eq!(detect_format_from_prefix(b"\x7fELF"), ExecutableFormat::Elf);
        assert_eq!(
            detect_format_from_prefix(b"#!/bin/sh"),
            ExecutableFormat::Script
        );
        assert_eq!(
            detect_format_from_prefix(b"text"),
            ExecutableFormat::Unknown
        );
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

    #[test]
    fn classifies_shebang_without_decoding_the_script_body() {
        let mut prefix = b"#!/usr/bin/env node\n".to_vec();
        prefix.extend([0xff, 0xfe]);

        assert_eq!(script_origin(&prefix), ExecutableOrigin::Linux);
    }
}
