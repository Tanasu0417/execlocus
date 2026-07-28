//! Explicit, injectable command-resolution contracts for supported shells.
//!
//! The production report does not use this module yet because a child process
//! cannot reliably reconstruct aliases, functions, cmdlets, builtins, or shell
//! hash state from environment variables alone. Callers must provide a bounded
//! [`ShellSessionSnapshot`]. An incomplete snapshot never produces a selected
//! command, even when external executable candidates are available.
//!
//! Contract sources:
//! - `PowerShell` command precedence and `Get-Command -All`:
//!   <https://learn.microsoft.com/powershell/module/microsoft.powershell.core/about/about_command_precedence>
//! - cmd current-directory and `PATH` order:
//!   <https://learn.microsoft.com/windows-server/administration/windows-commands/path>
//! - Bash command search:
//!   <https://www.gnu.org/software/bash/manual/html_node/Command-Search-and-Execution.html>
//! - zsh command execution:
//!   <https://zsh.sourceforge.io/Doc/Release/Command-Execution.html>

use std::path::PathBuf;

use crate::{
    model::{
        Confidence, Evidence, ExecutableCandidate, ExecutableFormat, ObservationStatus,
        ProbeFailure, ProbeResult, RuntimeKind,
    },
    probes::{
        context::ProbeContext,
        executable::{resolve_from_search_plan, windows_pathext_candidate_names},
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShellKind {
    PowerShell,
    Cmd,
    Bash,
    Zsh,
}

impl ShellKind {
    const fn contract_name(self) -> &'static str {
        match self {
            Self::PowerShell => "powershell",
            Self::Cmd => "cmd",
            Self::Bash => "bash",
            Self::Zsh => "zsh",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShellCommandKind {
    Alias,
    Function,
    Cmdlet,
    Builtin,
    ExternalScript,
    Application,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellCommandCandidate {
    pub kind: ShellCommandKind,
    pub name: String,
    /// A bounded identifier such as an alias target, module name, or path.
    /// Function bodies and other arbitrary shell source must not be stored.
    pub source: Option<String>,
    pub executable: Option<ExecutableCandidate>,
}

impl ShellCommandCandidate {
    #[must_use]
    pub fn session_binding(
        kind: ShellCommandKind,
        name: impl Into<String>,
        source: Option<String>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            source,
            executable: None,
        }
    }

    fn external(command: &str, executable: ExecutableCandidate) -> Self {
        let kind = if executable.format == ExecutableFormat::Script
            || executable
                .path
                .rsplit_once('.')
                .is_some_and(|(_, extension)| {
                    matches!(
                        extension.to_ascii_lowercase().as_str(),
                        "bat" | "cmd" | "ps1"
                    )
                }) {
            ShellCommandKind::ExternalScript
        } else {
            ShellCommandKind::Application
        };
        Self {
            kind,
            name: command.to_owned(),
            source: Some(executable.path.clone()),
            executable: Some(executable),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellSessionSnapshot {
    pub complete: bool,
    pub bindings: Vec<ShellCommandCandidate>,
}

impl ShellSessionSnapshot {
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            complete: false,
            bindings: Vec::new(),
        }
    }

    #[must_use]
    pub fn complete(bindings: Vec<ShellCommandCandidate>) -> Self {
        Self {
            complete: true,
            bindings,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellCommandResolution {
    pub shell: ShellKind,
    pub requested: String,
    pub selected: Option<ShellCommandCandidate>,
    pub candidates: Vec<ShellCommandCandidate>,
    pub session_complete: bool,
    pub status: ObservationStatus,
    pub confidence: Confidence,
}

#[must_use]
pub fn resolve_with_snapshot(
    context: &dyn ProbeContext,
    shell: ShellKind,
    command: &str,
    runtime: RuntimeKind,
    snapshot: &ShellSessionSnapshot,
) -> ProbeResult<ShellCommandResolution> {
    let mut failures = Vec::new();
    let directories = external_search_directories(context, shell, &mut failures);
    let names = external_candidate_names(context, shell, command, runtime);
    let external_result = resolve_from_search_plan(context, command, runtime, directories, &names);
    failures.extend(external_result.failures);

    let mut candidates = snapshot
        .bindings
        .iter()
        .filter(|candidate| command_names_match(shell, &candidate.name, command))
        .cloned()
        .collect::<Vec<_>>();
    for candidate in external_result
        .value
        .into_iter()
        .map(|candidate| ShellCommandCandidate::external(command, candidate))
    {
        if !candidates
            .iter()
            .any(|existing| existing.kind == candidate.kind && existing.source == candidate.source)
        {
            candidates.push(candidate);
        }
    }
    candidates.sort_by_key(|candidate| precedence(shell, candidate.kind));

    let selected = snapshot
        .complete
        .then(|| candidates.first().cloned())
        .flatten();
    let (status, confidence) = if !snapshot.complete {
        (ObservationStatus::Unavailable, Confidence::None)
    } else if selected.is_some() && failures.is_empty() {
        (ObservationStatus::Observed, Confidence::Certain)
    } else if selected.is_some() {
        (ObservationStatus::Observed, Confidence::High)
    } else if failures.is_empty() {
        (ObservationStatus::Unavailable, Confidence::None)
    } else {
        (ObservationStatus::Failed, Confidence::None)
    };

    let precedence_label = match shell {
        ShellKind::PowerShell => "alias > function > cmdlet > external-script > application",
        ShellKind::Cmd => "builtin > current-directory > PATH (PATHEXT order)",
        ShellKind::Bash | ShellKind::Zsh => "alias > function > builtin > PATH",
    };
    let evidence = vec![Evidence {
        id: format!("shell-resolution.{command}"),
        probe: "shell-resolution/v1".to_owned(),
        kind: "resolution-contract".to_owned(),
        claim: format!("{} command precedence contract", shell.contract_name()),
        value: Some(precedence_label.to_owned()),
        sensitive: false,
    }];

    ProbeResult {
        value: ShellCommandResolution {
            shell,
            requested: command.to_owned(),
            selected,
            candidates,
            session_complete: snapshot.complete,
            status,
            confidence,
        },
        evidence,
        failures,
    }
}

fn external_search_directories(
    context: &dyn ProbeContext,
    shell: ShellKind,
    failures: &mut Vec<ProbeFailure>,
) -> Vec<PathBuf> {
    let mut directories = Vec::new();
    if shell == ShellKind::Cmd {
        match context.current_dir() {
            Ok(current_dir) => directories.push(current_dir),
            Err(error) => failures.push(ProbeFailure {
                probe: "shell-resolution/v1".to_owned(),
                code: "CURRENT_DIRECTORY_UNAVAILABLE".to_owned(),
                message: format!("failed to observe the cmd current directory: {error}"),
            }),
        }
    }
    directories.extend(context.path_entries());
    directories
}

fn external_candidate_names(
    context: &dyn ProbeContext,
    shell: ShellKind,
    command: &str,
    runtime: RuntimeKind,
) -> Vec<String> {
    match (shell, runtime) {
        (ShellKind::PowerShell, RuntimeKind::WindowsNative) => {
            if command_has_extension(command) {
                vec![command.to_owned()]
            } else {
                let mut names = vec![format!("{command}.ps1")];
                names.extend(windows_pathext_candidate_names(context, command, false));
                names
            }
        }
        (ShellKind::PowerShell, _) if !command_has_extension(command) => {
            vec![format!("{command}.ps1"), command.to_owned()]
        }
        (ShellKind::Cmd, RuntimeKind::WindowsNative) => {
            windows_pathext_candidate_names(context, command, false)
        }
        _ => vec![command.to_owned()],
    }
}

fn command_has_extension(command: &str) -> bool {
    let file_name = command.rsplit(['\\', '/']).next().unwrap_or(command);
    file_name
        .rsplit_once('.')
        .is_some_and(|(stem, extension)| !stem.is_empty() && !extension.is_empty())
}

fn command_names_match(shell: ShellKind, observed: &str, requested: &str) -> bool {
    match shell {
        ShellKind::PowerShell | ShellKind::Cmd => observed.eq_ignore_ascii_case(requested),
        ShellKind::Bash | ShellKind::Zsh => observed == requested,
    }
}

const fn precedence(shell: ShellKind, kind: ShellCommandKind) -> usize {
    match shell {
        ShellKind::PowerShell => match kind {
            ShellCommandKind::Alias => 0,
            ShellCommandKind::Function => 1,
            ShellCommandKind::Cmdlet => 2,
            ShellCommandKind::ExternalScript => 3,
            ShellCommandKind::Application => 4,
            ShellCommandKind::Builtin => 5,
        },
        ShellKind::Cmd => match kind {
            ShellCommandKind::Builtin => 0,
            ShellCommandKind::ExternalScript | ShellCommandKind::Application => 1,
            ShellCommandKind::Alias | ShellCommandKind::Function | ShellCommandKind::Cmdlet => 2,
        },
        ShellKind::Bash | ShellKind::Zsh => match kind {
            ShellCommandKind::Alias => 0,
            ShellCommandKind::Function => 1,
            ShellCommandKind::Builtin => 2,
            ShellCommandKind::ExternalScript | ShellCommandKind::Application => 3,
            ShellCommandKind::Cmdlet => 4,
        },
    }
}
