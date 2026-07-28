use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use execlocus::{
    model::{Confidence, ObservationStatus, RuntimeKind},
    probes::{
        context::{CandidateSnapshot, HostPlatform, ProbeContext},
        shell::{
            ShellCommandCandidate, ShellCommandKind, ShellKind, ShellSessionSnapshot,
            resolve_with_snapshot,
        },
    },
};

struct ShellFixture {
    current_dir: PathBuf,
    path_entries: Vec<PathBuf>,
    environment: HashMap<String, String>,
    candidates: HashMap<(String, String), CandidateSnapshot>,
}

impl ShellFixture {
    fn windows() -> Self {
        Self {
            current_dir: PathBuf::from(r"C:\demo\project"),
            path_entries: vec![PathBuf::from(r"C:\Tools\bin")],
            environment: HashMap::from([("PATHEXT".to_owned(), ".CMD;.EXE".to_owned())]),
            candidates: HashMap::from([
                (
                    (r"C:\demo\project".to_owned(), "node.cmd".to_owned()),
                    CandidateSnapshot::new(
                        PathBuf::from(r"C:\demo\project\node.cmd"),
                        true,
                        b"@echo off".to_vec(),
                    ),
                ),
                (
                    (r"C:\Tools\bin".to_owned(), "node.exe".to_owned()),
                    CandidateSnapshot::new(
                        PathBuf::from(r"C:\Tools\bin\node.exe"),
                        true,
                        b"MZsynthetic".to_vec(),
                    ),
                ),
                (
                    (r"C:\Tools\bin".to_owned(), "node.ps1".to_owned()),
                    CandidateSnapshot::new(
                        PathBuf::from(r"C:\Tools\bin\node.ps1"),
                        true,
                        b"Write-Output synthetic".to_vec(),
                    ),
                ),
            ]),
        }
    }

    fn linux() -> Self {
        Self {
            current_dir: PathBuf::from("/demo/project"),
            path_entries: vec![PathBuf::from("/usr/local/bin"), PathBuf::from("/usr/bin")],
            environment: HashMap::new(),
            candidates: HashMap::from([(
                ("/usr/bin".to_owned(), "node".to_owned()),
                CandidateSnapshot::new(
                    PathBuf::from("/usr/bin/node"),
                    true,
                    b"\x7fELFsynthetic".to_vec(),
                ),
            )]),
        }
    }
}

impl ProbeContext for ShellFixture {
    fn host_platform(&self) -> HostPlatform {
        HostPlatform::Other
    }

    fn os_name(&self) -> String {
        "fixture".to_owned()
    }

    fn env_var(&self, key: &str) -> Option<String> {
        self.environment.get(key).cloned()
    }

    fn path_entries(&self) -> Vec<PathBuf> {
        self.path_entries.clone()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone())
    }

    fn inspect_candidate(
        &self,
        directory: &Path,
        name: &str,
        prefix_limit: usize,
    ) -> io::Result<Option<CandidateSnapshot>> {
        let key = (directory.to_string_lossy().into_owned(), name.to_owned());
        Ok(self.candidates.get(&key).cloned().map(|mut candidate| {
            candidate.prefix.truncate(prefix_limit);
            candidate
        }))
    }

    fn read_text(&self, _path: &Path, _max_bytes: usize) -> io::Result<String> {
        Err(io::Error::new(io::ErrorKind::NotFound, "missing fixture"))
    }

    fn now_unix_ms(&self) -> u128 {
        0
    }
}

#[test]
fn powershell_alias_wins_over_an_external_application() {
    let context = ShellFixture::windows();
    let snapshot = ShellSessionSnapshot::complete(vec![ShellCommandCandidate::session_binding(
        ShellCommandKind::Alias,
        "node",
        Some("Invoke-DemoNode".to_owned()),
    )]);

    let result = resolve_with_snapshot(
        &context,
        ShellKind::PowerShell,
        "node",
        RuntimeKind::WindowsNative,
        &snapshot,
    );

    assert!(result.failures.is_empty());
    assert_eq!(result.value.status, ObservationStatus::Observed);
    assert_eq!(result.value.confidence, Confidence::Certain);
    assert_eq!(
        result.value.selected.as_ref().map(|item| item.kind),
        Some(ShellCommandKind::Alias)
    );
    assert!(result.value.candidates.iter().any(|item| {
        item.kind == ShellCommandKind::Application
            && item.source.as_deref() == Some(r"C:\Tools\bin\node.exe")
    }));
    assert!(
        !result
            .value
            .candidates
            .iter()
            .any(|item| { item.source.as_deref() == Some(r"C:\demo\project\node.cmd") })
    );
}

#[test]
fn incomplete_powershell_snapshot_does_not_claim_the_external_candidate_wins() {
    let result = resolve_with_snapshot(
        &ShellFixture::windows(),
        ShellKind::PowerShell,
        "node",
        RuntimeKind::WindowsNative,
        &ShellSessionSnapshot::unavailable(),
    );

    assert!(result.value.selected.is_none());
    assert_eq!(result.value.status, ObservationStatus::Unavailable);
    assert_eq!(result.value.confidence, Confidence::None);
    assert!(
        result
            .value
            .candidates
            .iter()
            .any(|item| { item.source.as_deref() == Some(r"C:\Tools\bin\node.exe") })
    );
}

#[test]
fn powershell_external_script_precedes_pathext_application() {
    let snapshot = ShellSessionSnapshot::complete(vec![ShellCommandCandidate::session_binding(
        ShellCommandKind::Alias,
        "git",
        Some("Unrelated-Alias".to_owned()),
    )]);
    let result = resolve_with_snapshot(
        &ShellFixture::windows(),
        ShellKind::PowerShell,
        "node",
        RuntimeKind::WindowsNative,
        &snapshot,
    );

    assert_eq!(
        result
            .value
            .selected
            .as_ref()
            .and_then(|item| item.source.as_deref()),
        Some(r"C:\Tools\bin\node.ps1")
    );
    assert_eq!(
        result.value.selected.as_ref().map(|item| item.kind),
        Some(ShellCommandKind::ExternalScript)
    );
    assert!(
        !result
            .value
            .candidates
            .iter()
            .any(|item| item.name == "git")
    );
}

#[test]
fn cmd_searches_current_directory_then_path_using_pathext_order() {
    let result = resolve_with_snapshot(
        &ShellFixture::windows(),
        ShellKind::Cmd,
        "node",
        RuntimeKind::WindowsNative,
        &ShellSessionSnapshot::complete(Vec::new()),
    );

    assert!(result.failures.is_empty());
    assert_eq!(
        result
            .value
            .selected
            .as_ref()
            .and_then(|item| item.source.as_deref()),
        Some(r"C:\demo\project\node.cmd")
    );
    assert_eq!(result.value.candidates.len(), 2);
    assert_eq!(
        result.value.candidates[1].source.as_deref(),
        Some(r"C:\Tools\bin\node.exe")
    );
}

#[test]
fn bash_function_wins_before_path_search_result() {
    let snapshot = ShellSessionSnapshot::complete(vec![ShellCommandCandidate::session_binding(
        ShellCommandKind::Function,
        "node",
        Some("node".to_owned()),
    )]);
    let result = resolve_with_snapshot(
        &ShellFixture::linux(),
        ShellKind::Bash,
        "node",
        RuntimeKind::LinuxNative,
        &snapshot,
    );

    assert_eq!(
        result.value.selected.as_ref().map(|item| item.kind),
        Some(ShellCommandKind::Function)
    );
    assert_eq!(
        result.value.candidates[1].source.as_deref(),
        Some("/usr/bin/node")
    );
}

#[test]
fn zsh_builtin_wins_before_path_search_result() {
    let snapshot = ShellSessionSnapshot::complete(vec![ShellCommandCandidate::session_binding(
        ShellCommandKind::Builtin,
        "node",
        Some("zsh builtin".to_owned()),
    )]);
    let result = resolve_with_snapshot(
        &ShellFixture::linux(),
        ShellKind::Zsh,
        "node",
        RuntimeKind::LinuxNative,
        &snapshot,
    );

    assert_eq!(
        result.value.selected.as_ref().map(|item| item.kind),
        Some(ShellCommandKind::Builtin)
    );
    assert!(
        result
            .evidence
            .iter()
            .any(|item| item.value.as_deref() == Some("alias > function > builtin > PATH"))
    );
}
