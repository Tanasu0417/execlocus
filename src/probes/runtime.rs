use std::{env, fs};

use crate::model::{
    Confidence, Evidence, ObservationStatus, ProbeResult, RuntimeInfo, RuntimeKind,
};

#[must_use]
pub fn probe() -> ProbeResult<RuntimeInfo> {
    let kind = detect_runtime_kind();
    let os_name = match kind {
        RuntimeKind::WindowsNative => "Windows".to_owned(),
        RuntimeKind::Wsl => "WSL".to_owned(),
        RuntimeKind::LinuxNative => "Linux".to_owned(),
        RuntimeKind::Unknown => env::consts::OS.to_owned(),
    };
    let distribution = detect_distribution(kind);
    let user = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .ok()
        .filter(|value| !value.is_empty());
    let shell = env::var("SHELL")
        .or_else(|_| env::var("ComSpec"))
        .ok()
        .filter(|value| !value.is_empty());
    let terminal = detect_terminal();

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

fn detect_runtime_kind() -> RuntimeKind {
    if cfg!(windows) {
        return RuntimeKind::WindowsNative;
    }

    if cfg!(target_os = "linux") {
        if env::var_os("WSL_DISTRO_NAME").is_some()
            || env::var_os("WSL_INTEROP").is_some()
            || file_contains_microsoft("/proc/sys/kernel/osrelease")
            || file_contains_microsoft("/proc/version")
        {
            RuntimeKind::Wsl
        } else {
            RuntimeKind::LinuxNative
        }
    } else {
        RuntimeKind::Unknown
    }
}

fn file_contains_microsoft(path: &str) -> bool {
    fs::read_to_string(path).is_ok_and(|value| value.to_ascii_lowercase().contains("microsoft"))
}

fn detect_distribution(kind: RuntimeKind) -> Option<String> {
    if kind == RuntimeKind::Wsl {
        if let Ok(name) = env::var("WSL_DISTRO_NAME") {
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    if !matches!(kind, RuntimeKind::Wsl | RuntimeKind::LinuxNative) {
        return None;
    }

    let os_release = fs::read_to_string("/etc/os-release").ok()?;
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

fn detect_terminal() -> Option<String> {
    if env::var_os("WT_SESSION").is_some() {
        Some("Windows Terminal".to_owned())
    } else if env::var_os("TERM_PROGRAM").is_some() {
        env::var("TERM_PROGRAM").ok()
    } else if env::var_os("TERM").is_some() {
        env::var("TERM").ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::file_contains_microsoft;

    #[test]
    fn missing_kernel_file_is_not_microsoft() {
        assert!(!file_contains_microsoft("definitely-not-a-real-proc-file"));
    }
}
