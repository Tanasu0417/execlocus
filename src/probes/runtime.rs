use crate::model::{
    Confidence, Evidence, ObservationStatus, ProbeResult, RuntimeInfo, RuntimeKind,
};
use crate::probes::context::{HostPlatform, ProbeContext, SystemProbeContext};

#[must_use]
pub fn probe() -> ProbeResult<RuntimeInfo> {
    probe_with(&SystemProbeContext)
}

#[must_use]
pub fn probe_with<C>(context: &C) -> ProbeResult<RuntimeInfo>
where
    C: ProbeContext + ?Sized,
{
    let kind = detect_runtime_kind(context);
    let os_name = match kind {
        RuntimeKind::WindowsNative => "Windows".to_owned(),
        RuntimeKind::Wsl => "WSL".to_owned(),
        RuntimeKind::LinuxNative => "Linux".to_owned(),
        RuntimeKind::Unknown => context.os_name(),
    };
    let distribution = detect_distribution(context, kind);
    let user = context
        .env_var("USER")
        .or_else(|| context.env_var("USERNAME"))
        .filter(|value| !value.is_empty());
    let shell = context
        .env_var("SHELL")
        .or_else(|| context.env_var("ComSpec"))
        .filter(|value| !value.is_empty());
    let terminal = detect_terminal(context);

    let evidence = vec![
        Evidence {
            id: "runtime.kind".to_owned(),
            probe: "runtime/v1".to_owned(),
            kind: "environment".to_owned(),
            claim: "current process runtime".to_owned(),
            value: Some(format!("{kind:?}")),
            sensitive: false,
        },
        Evidence {
            id: "runtime.user".to_owned(),
            probe: "runtime/v1".to_owned(),
            kind: "environment".to_owned(),
            claim: "current user".to_owned(),
            value: user.clone(),
            sensitive: true,
        },
        Evidence {
            id: "runtime.shell".to_owned(),
            probe: "runtime/v1".to_owned(),
            kind: "environment".to_owned(),
            claim: "current shell hint".to_owned(),
            value: shell.clone(),
            sensitive: false,
        },
    ];

    ProbeResult {
        value: RuntimeInfo {
            kind,
            os_name,
            distribution,
            user,
            shell,
            terminal,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        },
        evidence,
        failures: Vec::new(),
    }
}

fn detect_runtime_kind<C>(context: &C) -> RuntimeKind
where
    C: ProbeContext + ?Sized,
{
    match context.host_platform() {
        HostPlatform::Windows => RuntimeKind::WindowsNative,
        HostPlatform::Linux => {
            if context.env_var("WSL_DISTRO_NAME").is_some()
                || context.env_var("WSL_INTEROP").is_some()
                || file_contains_microsoft(context, "/proc/sys/kernel/osrelease")
                || file_contains_microsoft(context, "/proc/version")
            {
                RuntimeKind::Wsl
            } else {
                RuntimeKind::LinuxNative
            }
        }
        HostPlatform::Other => RuntimeKind::Unknown,
    }
}

fn file_contains_microsoft<C>(context: &C, path: &str) -> bool
where
    C: ProbeContext + ?Sized,
{
    context
        .read_text(std::path::Path::new(path), 64 * 1024)
        .is_ok_and(|value| value.to_ascii_lowercase().contains("microsoft"))
}

fn detect_distribution<C>(context: &C, kind: RuntimeKind) -> Option<String>
where
    C: ProbeContext + ?Sized,
{
    if kind == RuntimeKind::Wsl {
        if let Some(name) = context.env_var("WSL_DISTRO_NAME") {
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    if !matches!(kind, RuntimeKind::Wsl | RuntimeKind::LinuxNative) {
        return None;
    }

    let os_release = context
        .read_text(std::path::Path::new("/etc/os-release"), 64 * 1024)
        .ok()?;
    for key in ["PRETTY_NAME", "NAME"] {
        if let Some(line) = os_release.lines().find(|line| line.starts_with(key)) {
            let value = line.split_once('=')?.1.trim_matches('"').trim();
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

fn detect_terminal<C>(context: &C) -> Option<String>
where
    C: ProbeContext + ?Sized,
{
    if context.env_var("WT_SESSION").is_some() {
        Some("Windows Terminal".to_owned())
    } else if context.env_var("TERM_PROGRAM").is_some() {
        context.env_var("TERM_PROGRAM")
    } else if context.env_var("TERM").is_some() {
        context.env_var("TERM")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::file_contains_microsoft;
    use crate::probes::context::{CandidateSnapshot, HostPlatform, ProbeContext};
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

        fn env_var(&self, _key: &str) -> Option<String> {
            None
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

    #[test]
    fn missing_kernel_file_is_not_microsoft() {
        assert!(!file_contains_microsoft(
            &MissingFileContext,
            "definitely-not-a-real-proc-file"
        ));
    }
}
