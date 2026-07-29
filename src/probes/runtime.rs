use std::path::Path;

use crate::model::{
    Confidence, Evidence, ObservationStatus, ProbeFailure, ProbeResult, RuntimeInfo, RuntimeKind,
    RuntimeValueSource,
};
use crate::probes::{
    context::{HostPlatform, ProbeContext, SystemProbeContext},
    process::{
        MAX_PROCESS_ANCESTRY, ProcessRecord, RuntimeIdentityInspector, RuntimeIdentitySnapshot,
        SystemRuntimeIdentityInspector, snapshot_failure,
    },
};

struct DetectedValue {
    value: String,
    source: RuntimeValueSource,
}

#[derive(Clone, Copy)]
struct DetectedRuntime {
    kind: RuntimeKind,
    source: Option<RuntimeValueSource>,
    status: ObservationStatus,
    confidence: Confidence,
}

#[must_use]
pub fn probe() -> ProbeResult<RuntimeInfo> {
    probe_with_inspector(&SystemProbeContext, &SystemRuntimeIdentityInspector)
}

/// Runs deterministic environment and filesystem probes only.
///
/// Injected test contexts use this entry point so they never inspect the test
/// runner's real account or process tree.
#[must_use]
pub fn probe_with<C>(context: &C) -> ProbeResult<RuntimeInfo>
where
    C: ProbeContext + ?Sized,
{
    probe_with_identity(context, None)
}

#[must_use]
pub fn probe_with_identity<C>(
    context: &C,
    identity: Option<&RuntimeIdentitySnapshot>,
) -> ProbeResult<RuntimeInfo>
where
    C: ProbeContext + ?Sized,
{
    probe_internal(context, identity, Vec::new())
}

#[must_use]
pub fn probe_with_inspector<C>(
    context: &C,
    inspector: &dyn RuntimeIdentityInspector,
) -> ProbeResult<RuntimeInfo>
where
    C: ProbeContext + ?Sized,
{
    match inspector.inspect(MAX_PROCESS_ANCESTRY) {
        Ok(identity) => probe_internal(context, Some(&identity), Vec::new()),
        Err(error) => probe_internal(context, None, vec![snapshot_failure(&error)]),
    }
}

fn probe_internal<C>(
    context: &C,
    identity: Option<&RuntimeIdentitySnapshot>,
    failures: Vec<ProbeFailure>,
) -> ProbeResult<RuntimeInfo>
where
    C: ProbeContext + ?Sized,
{
    let runtime = detect_runtime(context);
    let os_name = match runtime.kind {
        RuntimeKind::WindowsNative => "Windows".to_owned(),
        RuntimeKind::Wsl => "WSL".to_owned(),
        RuntimeKind::LinuxNative => "Linux".to_owned(),
        RuntimeKind::Unknown => context.os_name(),
    };
    let distribution = detect_distribution(context, runtime.kind);
    let user = detect_user(context, identity);
    let shell = detect_shell(context, identity);
    let terminal = detect_terminal(context);

    let evidence = build_runtime_evidence(
        runtime,
        distribution.as_ref(),
        user.as_ref(),
        shell.as_ref(),
        terminal.as_deref(),
    );

    ProbeResult {
        value: RuntimeInfo {
            kind: runtime.kind,
            kind_source: runtime.source,
            os_name,
            distribution: distribution.as_ref().map(|observed| observed.value.clone()),
            distribution_source: distribution.as_ref().map(|observed| observed.source),
            user: user.as_ref().map(|observed| observed.value.clone()),
            user_source: user.as_ref().map(|observed| observed.source),
            shell: shell.as_ref().map(|observed| observed.value.clone()),
            shell_source: shell.as_ref().map(|observed| observed.source),
            terminal,
            status: runtime.status,
            confidence: runtime.confidence,
        },
        evidence,
        failures,
    }
}

fn build_runtime_evidence(
    runtime: DetectedRuntime,
    distribution: Option<&DetectedValue>,
    user: Option<&DetectedValue>,
    shell: Option<&DetectedValue>,
    terminal: Option<&str>,
) -> Vec<Evidence> {
    let mut evidence = vec![Evidence {
        id: "runtime.kind".to_owned(),
        probe: "runtime/v2".to_owned(),
        kind: runtime.source.map_or("unavailable", source_kind).to_owned(),
        claim: "current ExecLocus process runtime".to_owned(),
        value: (runtime.kind != RuntimeKind::Unknown).then(|| format!("{:?}", runtime.kind)),
        sensitive: false,
    }];
    if let Some(observed) = distribution {
        evidence.push(Evidence {
            id: "runtime.distribution".to_owned(),
            probe: "runtime/v2".to_owned(),
            kind: source_kind(observed.source).to_owned(),
            claim: "current Linux or WSL distribution".to_owned(),
            value: Some(observed.value.clone()),
            sensitive: false,
        });
    }
    evidence.push(Evidence {
        id: "runtime.user".to_owned(),
        probe: "runtime/v2".to_owned(),
        kind: user
            .map_or("environment", |observed| source_kind(observed.source))
            .to_owned(),
        claim: user.map_or_else(
            || "current user unavailable".to_owned(),
            |observed| match observed.source {
                RuntimeValueSource::OsAccount => "current OS account".to_owned(),
                _ => "current user environment hint".to_owned(),
            },
        ),
        value: user.map(|observed| observed.value.clone()),
        sensitive: true,
    });
    evidence.push(Evidence {
        id: "runtime.shell".to_owned(),
        probe: "runtime/v2".to_owned(),
        kind: shell
            .map_or("environment", |observed| source_kind(observed.source))
            .to_owned(),
        claim: shell.map_or_else(
            || "launching shell unavailable".to_owned(),
            |observed| match observed.source {
                RuntimeValueSource::ProcessAncestry => {
                    "launching shell found in process ancestry".to_owned()
                }
                _ => "default shell environment hint".to_owned(),
            },
        ),
        value: shell.map(|observed| observed.value.clone()),
        sensitive: false,
    });
    if let Some(value) = terminal {
        evidence.push(Evidence {
            id: "runtime.terminal".to_owned(),
            probe: "runtime/v2".to_owned(),
            kind: "environment".to_owned(),
            claim: "visible terminal environment hint".to_owned(),
            value: Some(value.to_owned()),
            sensitive: false,
        });
    }
    evidence
}

fn detect_runtime<C>(context: &C) -> DetectedRuntime
where
    C: ProbeContext + ?Sized,
{
    match context.host_platform() {
        HostPlatform::Windows => DetectedRuntime {
            kind: RuntimeKind::WindowsNative,
            source: Some(RuntimeValueSource::TargetPlatform),
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        },
        HostPlatform::Linux => match kernel_mentions_microsoft(context) {
            Some(true) => DetectedRuntime {
                kind: RuntimeKind::Wsl,
                source: Some(RuntimeValueSource::KernelRelease),
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            Some(false) => DetectedRuntime {
                kind: RuntimeKind::LinuxNative,
                source: Some(RuntimeValueSource::KernelRelease),
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            None if nonempty_env(context, "WSL_DISTRO_NAME").is_some()
                || nonempty_env(context, "WSL_INTEROP").is_some() =>
            {
                DetectedRuntime {
                    kind: RuntimeKind::Wsl,
                    source: Some(RuntimeValueSource::Environment),
                    status: ObservationStatus::Inferred,
                    confidence: Confidence::High,
                }
            }
            None => DetectedRuntime {
                kind: RuntimeKind::Unknown,
                source: None,
                status: ObservationStatus::Unavailable,
                confidence: Confidence::None,
            },
        },
        HostPlatform::Other => DetectedRuntime {
            kind: RuntimeKind::Unknown,
            source: None,
            status: ObservationStatus::Unavailable,
            confidence: Confidence::None,
        },
    }
}

fn kernel_mentions_microsoft<C>(context: &C) -> Option<bool>
where
    C: ProbeContext + ?Sized,
{
    let mut observed = false;
    for path in ["/proc/sys/kernel/osrelease", "/proc/version"] {
        if let Ok(value) = context.read_text(Path::new(path), 64 * 1024) {
            observed = true;
            if value.to_ascii_lowercase().contains("microsoft") {
                return Some(true);
            }
        }
    }
    observed.then_some(false)
}

fn detect_distribution<C>(context: &C, kind: RuntimeKind) -> Option<DetectedValue>
where
    C: ProbeContext + ?Sized,
{
    if kind == RuntimeKind::Wsl {
        if let Some(value) = nonempty_env(context, "WSL_DISTRO_NAME") {
            return Some(DetectedValue {
                value,
                source: RuntimeValueSource::Environment,
            });
        }
    }

    if !matches!(kind, RuntimeKind::Wsl | RuntimeKind::LinuxNative) {
        return None;
    }

    let os_release = context
        .read_text(Path::new("/etc/os-release"), 64 * 1024)
        .ok()?;
    for key in ["PRETTY_NAME", "NAME"] {
        let prefix = format!("{key}=");
        if let Some(value) = os_release.lines().find_map(|line| {
            line.strip_prefix(&prefix)
                .map(|value| value.trim_matches('"').trim())
                .filter(|value| !value.is_empty())
        }) {
            return Some(DetectedValue {
                value: value.to_owned(),
                source: RuntimeValueSource::OsRelease,
            });
        }
    }
    None
}

fn detect_user<C>(context: &C, identity: Option<&RuntimeIdentitySnapshot>) -> Option<DetectedValue>
where
    C: ProbeContext + ?Sized,
{
    if let Some(value) = identity
        .and_then(|snapshot| snapshot.user.as_deref())
        .filter(|value| !value.is_empty())
    {
        return Some(DetectedValue {
            value: value.to_owned(),
            source: RuntimeValueSource::OsAccount,
        });
    }

    nonempty_env(context, "USER")
        .or_else(|| nonempty_env(context, "USERNAME"))
        .map(|value| DetectedValue {
            value,
            source: RuntimeValueSource::Environment,
        })
}

fn detect_shell<C>(context: &C, identity: Option<&RuntimeIdentitySnapshot>) -> Option<DetectedValue>
where
    C: ProbeContext + ?Sized,
{
    if let Some(value) = identity
        .into_iter()
        .flat_map(|snapshot| &snapshot.process_ancestry)
        .find_map(known_shell)
    {
        return Some(DetectedValue {
            value,
            source: RuntimeValueSource::ProcessAncestry,
        });
    }

    nonempty_env(context, "SHELL")
        .or_else(|| nonempty_env(context, "ComSpec"))
        .map(|value| DetectedValue {
            value,
            source: RuntimeValueSource::Environment,
        })
}

fn known_shell(record: &ProcessRecord) -> Option<String> {
    let normalized = record.name.trim().to_ascii_lowercase();
    let normalized = normalized.strip_suffix(".exe").unwrap_or(&normalized);
    match normalized {
        "powershell" => Some("PowerShell".to_owned()),
        "pwsh" => Some("PowerShell 7".to_owned()),
        "cmd" => Some("cmd".to_owned()),
        "bash" => Some("bash".to_owned()),
        "zsh" => Some("zsh".to_owned()),
        "fish" => Some("fish".to_owned()),
        "dash" => Some("dash".to_owned()),
        "sh" => Some("sh".to_owned()),
        _ => None,
    }
}

fn detect_terminal<C>(context: &C) -> Option<String>
where
    C: ProbeContext + ?Sized,
{
    if nonempty_env(context, "WT_SESSION").is_some() {
        Some("Windows Terminal".to_owned())
    } else {
        nonempty_env(context, "TERM_PROGRAM").or_else(|| nonempty_env(context, "TERM"))
    }
}

fn nonempty_env<C>(context: &C, key: &str) -> Option<String>
where
    C: ProbeContext + ?Sized,
{
    context.env_var(key).filter(|value| !value.is_empty())
}

const fn source_kind(source: RuntimeValueSource) -> &'static str {
    match source {
        RuntimeValueSource::TargetPlatform => "target-platform",
        RuntimeValueSource::KernelRelease => "kernel",
        RuntimeValueSource::ProcessAncestry => "process",
        RuntimeValueSource::OsAccount => "os-account",
        RuntimeValueSource::Environment => "environment",
        RuntimeValueSource::OsRelease => "filesystem",
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_runtime, kernel_mentions_microsoft, known_shell, probe_with_inspector};
    use crate::{
        model::{Confidence, ObservationStatus, RuntimeKind, RuntimeValueSource},
        probes::{
            context::{CandidateSnapshot, HostPlatform, ProbeContext},
            process::{ProcessRecord, RuntimeIdentityInspector, RuntimeIdentitySnapshot},
        },
    };
    use std::{
        io,
        path::{Path, PathBuf},
    };

    struct MissingFileContext;

    impl ProbeContext for MissingFileContext {
        fn host_platform(&self) -> HostPlatform {
            HostPlatform::Linux
        }

        fn os_name(&self) -> String {
            "linux".to_owned()
        }

        fn env_var(&self, key: &str) -> Option<String> {
            match key {
                "USER" => Some("environment-user".to_owned()),
                "SHELL" => Some("/bin/environment-shell".to_owned()),
                _ => None,
            }
        }

        fn path_entries(&self) -> Vec<PathBuf> {
            Vec::new()
        }

        fn current_dir(&self) -> io::Result<PathBuf> {
            Ok(PathBuf::from("/demo"))
        }

        fn inspect_candidate(
            &self,
            _directory: &Path,
            _name: &str,
            _prefix_limit: usize,
        ) -> io::Result<Option<CandidateSnapshot>> {
            Ok(None)
        }

        fn read_text(&self, _path: &Path, _max_bytes: usize) -> io::Result<String> {
            Err(io::Error::new(io::ErrorKind::NotFound, "missing fixture"))
        }

        fn now_unix_ms(&self) -> u128 {
            0
        }
    }

    struct StaticIdentityInspector;

    impl RuntimeIdentityInspector for StaticIdentityInspector {
        fn inspect(&self, _max_depth: usize) -> io::Result<RuntimeIdentitySnapshot> {
            Ok(RuntimeIdentitySnapshot {
                user: Some("os-user".to_owned()),
                process_ancestry: vec![
                    ProcessRecord {
                        pid: 30,
                        parent_pid: 20,
                        name: "execlocus".to_owned(),
                    },
                    ProcessRecord {
                        pid: 20,
                        parent_pid: 10,
                        name: "cargo".to_owned(),
                    },
                    ProcessRecord {
                        pid: 10,
                        parent_pid: 1,
                        name: "bash".to_owned(),
                    },
                ],
            })
        }
    }

    #[test]
    fn missing_kernel_file_is_not_microsoft() {
        assert_eq!(kernel_mentions_microsoft(&MissingFileContext), None);
    }

    #[test]
    fn process_and_os_identity_override_environment_hints() {
        let result = probe_with_inspector(&MissingFileContext, &StaticIdentityInspector);

        assert_eq!(result.value.kind, RuntimeKind::Unknown);
        assert_eq!(result.value.user.as_deref(), Some("os-user"));
        assert_eq!(
            result.value.user_source,
            Some(RuntimeValueSource::OsAccount)
        );
        assert_eq!(result.value.shell.as_deref(), Some("bash"));
        assert_eq!(
            result.value.shell_source,
            Some(RuntimeValueSource::ProcessAncestry)
        );
        assert!(result.failures.is_empty());
    }

    struct FailingIdentityInspector;

    impl RuntimeIdentityInspector for FailingIdentityInspector {
        fn inspect(&self, _max_depth: usize) -> io::Result<RuntimeIdentitySnapshot> {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "fixture denied",
            ))
        }
    }

    #[test]
    fn identity_failure_falls_back_to_environment_hints() {
        let result = probe_with_inspector(&MissingFileContext, &FailingIdentityInspector);

        assert_eq!(result.value.user.as_deref(), Some("environment-user"));
        assert_eq!(
            result.value.user_source,
            Some(RuntimeValueSource::Environment)
        );
        assert_eq!(
            result.value.shell.as_deref(),
            Some("/bin/environment-shell")
        );
        assert_eq!(
            result.value.shell_source,
            Some(RuntimeValueSource::Environment)
        );
        assert_eq!(result.failures.len(), 1);
        assert_eq!(result.failures[0].code, "PROCESS_IDENTITY_SNAPSHOT_FAILED");
    }

    #[test]
    fn windows_shell_matching_is_case_insensitive() {
        let record = ProcessRecord {
            pid: 1,
            parent_pid: 0,
            name: "PWSH.EXE".to_owned(),
        };

        assert_eq!(known_shell(&record).as_deref(), Some("PowerShell 7"));
    }

    struct SpoofedWslEnvironmentContext;

    impl ProbeContext for SpoofedWslEnvironmentContext {
        fn host_platform(&self) -> HostPlatform {
            HostPlatform::Linux
        }

        fn os_name(&self) -> String {
            "linux".to_owned()
        }

        fn env_var(&self, key: &str) -> Option<String> {
            (key == "WSL_DISTRO_NAME").then(|| "spoofed".to_owned())
        }

        fn path_entries(&self) -> Vec<PathBuf> {
            Vec::new()
        }

        fn current_dir(&self) -> io::Result<PathBuf> {
            Ok(PathBuf::from("/demo"))
        }

        fn inspect_candidate(
            &self,
            _directory: &Path,
            _name: &str,
            _prefix_limit: usize,
        ) -> io::Result<Option<CandidateSnapshot>> {
            Ok(None)
        }

        fn read_text(&self, path: &Path, _max_bytes: usize) -> io::Result<String> {
            match path.to_string_lossy().as_ref() {
                "/proc/sys/kernel/osrelease" => Ok("6.8.0-generic".to_owned()),
                _ => Err(io::Error::new(io::ErrorKind::NotFound, "missing fixture")),
            }
        }

        fn now_unix_ms(&self) -> u128 {
            0
        }
    }

    #[test]
    fn kernel_evidence_wins_over_a_spoofed_wsl_environment_hint() {
        let runtime = detect_runtime(&SpoofedWslEnvironmentContext);

        assert_eq!(runtime.kind, RuntimeKind::LinuxNative);
        assert_eq!(runtime.source, Some(RuntimeValueSource::KernelRelease));
        assert_eq!(runtime.status, ObservationStatus::Observed);
        assert_eq!(runtime.confidence, Confidence::Certain);
    }
}
