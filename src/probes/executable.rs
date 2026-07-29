use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{
    model::{
        Confidence, Evidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo,
        ExecutableOrigin, ExecutableResolutionMethod, ExecutableSelectionKind, ObservationStatus,
        ProbeFailure, ProbeResult, RuntimeKind, ToolchainState,
    },
    probes::context::{CandidateSnapshot, ProbeContext, SystemProbeContext},
    probes::path::classify_path,
    probes::shell::{ShellKind, ShellSessionSnapshot, resolve_with_snapshot},
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
        evidence,
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
    let selected_kind = selected.as_ref().map(|candidate| match candidate.format {
        ExecutableFormat::Script => ExecutableSelectionKind::ExternalScript,
        ExecutableFormat::Pe | ExecutableFormat::Elf | ExecutableFormat::Unknown => {
            ExecutableSelectionKind::Application
        }
    });

    build_probe_result(
        role,
        command,
        selected,
        selected_kind,
        None,
        candidates,
        ExecutableResolutionMethod::PathFallback,
        None,
        None,
        status,
        confidence,
        evidence,
        failures,
    )
}

#[must_use]
pub fn probe_with_shell_snapshot(
    context: &dyn ProbeContext,
    role: &str,
    command: &str,
    runtime: &RuntimeKind,
    shell: ShellKind,
    snapshot: &ShellSessionSnapshot,
) -> ProbeResult<ExecutableInfo> {
    let ProbeResult {
        value: resolution,
        evidence,
        failures,
    } = resolve_with_snapshot(context, shell, command, *runtime, snapshot);
    let selected = resolution
        .selected
        .as_ref()
        .and_then(|candidate| candidate.executable.clone());
    let selected_kind = resolution
        .selected
        .as_ref()
        .map(|candidate| selection_kind(candidate.kind));
    let selected_binding = resolution.selected.as_ref().and_then(|candidate| {
        candidate.executable.is_none().then(|| {
            candidate
                .source
                .clone()
                .unwrap_or_else(|| candidate.name.clone())
        })
    });
    let candidates = resolution
        .candidates
        .iter()
        .filter_map(|candidate| candidate.executable.clone())
        .collect::<Vec<_>>();

    build_probe_result(
        role,
        command,
        selected,
        selected_kind,
        selected_binding,
        candidates,
        ExecutableResolutionMethod::ShellContract,
        Some(shell.contract_name().to_owned()),
        Some(resolution.session_complete),
        resolution.status,
        resolution.confidence,
        evidence,
        failures,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_probe_result(
    role: &str,
    command: &str,
    selected: Option<ExecutableCandidate>,
    selected_kind: Option<ExecutableSelectionKind>,
    selected_binding: Option<String>,
    candidates: Vec<ExecutableCandidate>,
    resolution_method: ExecutableResolutionMethod,
    resolution_shell: Option<String>,
    shell_session_complete: Option<bool>,
    status: ObservationStatus,
    confidence: Confidence,
    mut evidence: Vec<Evidence>,
    failures: Vec<ProbeFailure>,
) -> ProbeResult<ExecutableInfo> {
    let selection_state =
        selection_state(selected_kind, candidates.is_empty(), failures.is_empty());
    let selection_reason = selection_reason(
        selection_state,
        resolution_method,
        resolution_shell.as_deref(),
        shell_session_complete,
    );
    let verification_command =
        verification_command(command, resolution_method, resolution_shell.as_deref());
    evidence.push(Evidence {
        id: format!("executable.{role}.resolution"),
        probe: "executable/v2".to_owned(),
        kind: "resolution-method".to_owned(),
        claim: match resolution_method {
            ExecutableResolutionMethod::ShellContract => format!(
                "{} shell contract with {} session evidence",
                resolution_shell.as_deref().unwrap_or("unknown"),
                if shell_session_complete == Some(true) {
                    "complete"
                } else {
                    "incomplete"
                }
            ),
            ExecutableResolutionMethod::PathFallback => {
                "generic PATH resolution fallback".to_owned()
            }
        },
        value: None,
        sensitive: false,
    });

    if let Some(candidate) = &selected {
        evidence.push(Evidence {
            id: format!("executable.{role}"),
            probe: "executable/v2".to_owned(),
            kind: "executable".to_owned(),
            claim: format!("{role} resolves to {:?} executable", candidate.origin),
            value: Some(candidate.path.clone()),
            sensitive: true,
        });
    }
    for (index, candidate) in candidates.iter().enumerate() {
        evidence.push(Evidence {
            id: format!("executable.{role}.candidate.{}", index + 1),
            probe: "executable/v2".to_owned(),
            kind: "executable-candidate".to_owned(),
            claim: format!(
                "{role} candidate {} has {:?} origin",
                index + 1,
                candidate.origin
            ),
            value: Some(candidate.path.clone()),
            sensitive: true,
        });
    }

    ProbeResult {
        value: ExecutableInfo {
            role: role.to_owned(),
            requested: command.to_owned(),
            selection_state,
            selected,
            selected_kind,
            selected_binding,
            candidates,
            resolution_method,
            resolution_shell,
            shell_session_complete,
            selection_reason,
            verification_command,
            status,
            confidence,
        },
        evidence,
        failures,
    }
}

const fn selection_kind(kind: crate::probes::shell::ShellCommandKind) -> ExecutableSelectionKind {
    match kind {
        crate::probes::shell::ShellCommandKind::Alias => ExecutableSelectionKind::Alias,
        crate::probes::shell::ShellCommandKind::Function => ExecutableSelectionKind::Function,
        crate::probes::shell::ShellCommandKind::Cmdlet => ExecutableSelectionKind::Cmdlet,
        crate::probes::shell::ShellCommandKind::Builtin => ExecutableSelectionKind::Builtin,
        crate::probes::shell::ShellCommandKind::ExternalScript => {
            ExecutableSelectionKind::ExternalScript
        }
        crate::probes::shell::ShellCommandKind::Application => ExecutableSelectionKind::Application,
    }
}

const fn selection_state(
    selected_kind: Option<ExecutableSelectionKind>,
    candidates_empty: bool,
    failures_empty: bool,
) -> ToolchainState {
    if selected_kind.is_some() {
        ToolchainState::Selected
    } else if !candidates_empty {
        ToolchainState::CandidatesUnconfirmed
    } else if failures_empty {
        ToolchainState::NotFound
    } else {
        ToolchainState::ProbeFailed
    }
}

fn selection_reason(
    state: ToolchainState,
    method: ExecutableResolutionMethod,
    shell: Option<&str>,
    session_complete: Option<bool>,
) -> String {
    match state {
        ToolchainState::Selected if method == ExecutableResolutionMethod::PathFallback => {
            "generic PATH order selected the first inspected executable".to_owned()
        }
        ToolchainState::Selected => format!(
            "complete {} session evidence established command precedence",
            shell.unwrap_or("shell")
        ),
        ToolchainState::CandidatesUnconfirmed if session_complete == Some(false) => format!(
            "external candidates were found, but parent {} aliases, functions, or builtins were not captured",
            shell.unwrap_or("shell")
        ),
        ToolchainState::CandidatesUnconfirmed => {
            "candidates were found, but probe evidence was incomplete".to_owned()
        }
        ToolchainState::NotFound => "no executable candidate or shell binding was found".to_owned(),
        ToolchainState::ProbeFailed => {
            "candidate probing failed before a reliable selection could be established".to_owned()
        }
    }
}

fn verification_command(
    command: &str,
    method: ExecutableResolutionMethod,
    shell: Option<&str>,
) -> String {
    match (method, shell) {
        (ExecutableResolutionMethod::ShellContract, Some("powershell")) => format!(
            "Get-Command -All {command} | Select-Object CommandType, Name, Source, Definition"
        ),
        (ExecutableResolutionMethod::ShellContract, Some("cmd")) => {
            format!("where.exe {command}")
        }
        (ExecutableResolutionMethod::ShellContract, Some("bash" | "zsh")) => {
            format!("type -a -- {command}")
        }
        _ => format!("inspect PATH candidates for {command}"),
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
    resolve_from_search_plan(context, command, runtime, context.path_entries(), &names)
}

pub(super) fn resolve_from_search_plan(
    context: &dyn ProbeContext,
    command: &str,
    runtime: RuntimeKind,
    directories: Vec<PathBuf>,
    names: &[String],
) -> ProbeResult<Vec<ExecutableCandidate>> {
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    let mut failures = Vec::new();

    for directory in directories {
        for name in names {
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

    let pathext = context
        .env_var("PATHEXT")
        .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".to_owned());
    let extensions = pathext
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

pub(super) fn windows_pathext_candidate_names(
    context: &dyn ProbeContext,
    command: &str,
    include_bare_name: bool,
) -> Vec<String> {
    if command_has_extension(command, RuntimeKind::WindowsNative) {
        return vec![command.to_owned()];
    }

    let pathext = context
        .env_var("PATHEXT")
        .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".to_owned());
    let extensions = pathext
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase);
    let mut names = Vec::new();
    if include_bare_name {
        names.push(command.to_owned());
    }
    names.extend(extensions.map(|extension| format!("{command}{extension}")));
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
        ExecutableFormat::Script => script_origin(&snapshot.prefix, runtime),
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

fn script_origin(prefix: &[u8], runtime: RuntimeKind) -> ExecutableOrigin {
    let line_end = prefix
        .iter()
        .position(|byte| *byte == b'\n')
        .unwrap_or(prefix.len());
    let first_line = String::from_utf8_lossy(&prefix[..line_end]).to_ascii_lowercase();
    if first_line.contains("powershell") || first_line.contains("cmd.exe") {
        ExecutableOrigin::Windows
    } else if matches!(runtime, RuntimeKind::Wsl | RuntimeKind::LinuxNative)
        && (first_line.contains("/bin/") || first_line.contains("/usr/bin/"))
    {
        ExecutableOrigin::Linux
    } else {
        ExecutableOrigin::Script
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_format_from_prefix, display_path, script_origin, selection_state};
    use crate::model::{
        ExecutableFormat, ExecutableOrigin, ExecutableSelectionKind, RuntimeKind, ToolchainState,
    };

    #[test]
    fn toolchain_states_are_mutually_distinct() {
        assert_eq!(
            selection_state(Some(ExecutableSelectionKind::Application), false, true),
            ToolchainState::Selected
        );
        assert_eq!(
            selection_state(None, false, true),
            ToolchainState::CandidatesUnconfirmed
        );
        assert_eq!(selection_state(None, true, true), ToolchainState::NotFound);
        assert_eq!(
            selection_state(None, true, false),
            ToolchainState::ProbeFailed
        );
    }

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
    fn malformed_and_truncated_headers_remain_unknown_without_panicking() {
        for prefix in [b"".as_slice(), b"M", b"\x7fEL", b"not-a-header"] {
            assert_eq!(detect_format_from_prefix(prefix), ExecutableFormat::Unknown);
        }
        assert_eq!(
            detect_format_from_prefix(b"#!\xff\xfe"),
            ExecutableFormat::Script
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

        assert_eq!(
            script_origin(&prefix, RuntimeKind::Wsl),
            ExecutableOrigin::Linux
        );
        assert_eq!(
            script_origin(&prefix, RuntimeKind::WindowsNative),
            ExecutableOrigin::Script
        );
    }
}
