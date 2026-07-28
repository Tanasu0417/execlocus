use std::{
    env, fs,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HostPlatform {
    Windows,
    Linux,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct CandidateSnapshot {
    pub resolved_path: PathBuf,
    pub executable: bool,
    pub prefix: Vec<u8>,
}

impl CandidateSnapshot {
    #[must_use]
    pub fn new(resolved_path: PathBuf, executable: bool, prefix: Vec<u8>) -> Self {
        Self {
            resolved_path,
            executable,
            prefix,
        }
    }
}

/// Supplies the process and filesystem facts consumed by probes.
///
/// Production uses [`SystemProbeContext`]. Synthetic implementations can
/// provide deterministic facts without changing process-global environment
/// variables or depending on the maintainer's filesystem. This is a pre-alpha
/// extension seam; new capability traits will be added instead of expanding
/// this trait when shell-specific probing needs additional inputs.
pub trait ProbeContext {
    fn host_platform(&self) -> HostPlatform;

    fn os_name(&self) -> String;

    fn env_var(&self, key: &str) -> Option<String>;

    fn path_entries(&self) -> Vec<PathBuf>;

    /// Returns the project directory observed by the probe.
    ///
    /// # Errors
    ///
    /// Returns an error when the directory cannot be observed.
    fn current_dir(&self) -> io::Result<PathBuf>;

    /// Inspects one command name within one ordered search directory.
    ///
    /// # Errors
    ///
    /// Returns an error when an existing candidate cannot be inspected.
    fn inspect_candidate(
        &self,
        directory: &Path,
        name: &str,
        prefix_limit: usize,
    ) -> io::Result<Option<CandidateSnapshot>>;

    /// Reads at most `max_bytes` of a small text evidence file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read. Invalid or truncated
    /// UTF-8 is replaced so a bounded read cannot split a valid character and
    /// discard all preceding evidence.
    fn read_text(&self, path: &Path, max_bytes: usize) -> io::Result<String>;

    fn now_unix_ms(&self) -> u128;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemProbeContext;

impl ProbeContext for SystemProbeContext {
    fn host_platform(&self) -> HostPlatform {
        if cfg!(windows) {
            HostPlatform::Windows
        } else if cfg!(target_os = "linux") {
            HostPlatform::Linux
        } else {
            HostPlatform::Other
        }
    }

    fn os_name(&self) -> String {
        env::consts::OS.to_owned()
    }

    fn env_var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }

    fn path_entries(&self) -> Vec<PathBuf> {
        env::var_os("PATH")
            .map(|value| env::split_paths(&value).collect())
            .unwrap_or_default()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        env::current_dir()
    }

    fn inspect_candidate(
        &self,
        directory: &Path,
        name: &str,
        prefix_limit: usize,
    ) -> io::Result<Option<CandidateSnapshot>> {
        let path = directory.join(name);
        let metadata = match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => metadata,
            Ok(_) => return Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };

        let resolved_path = fs::canonicalize(path)?;
        let executable = system_metadata_is_executable(&metadata);
        let prefix = if executable {
            read_prefix(&resolved_path, prefix_limit)?
        } else {
            Vec::new()
        };

        Ok(Some(CandidateSnapshot {
            resolved_path,
            executable,
            prefix,
        }))
    }

    fn read_text(&self, path: &Path, max_bytes: usize) -> io::Result<String> {
        let bytes = read_prefix(path, max_bytes)?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    fn now_unix_ms(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_millis())
    }
}

fn read_prefix(path: &Path, limit: usize) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut bytes = Vec::with_capacity(limit.min(4096));
    file.take(limit as u64).read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(unix)]
fn system_metadata_is_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;

    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(windows)]
fn system_metadata_is_executable(_metadata: &fs::Metadata) -> bool {
    true
}

#[cfg(not(any(unix, windows)))]
fn system_metadata_is_executable(_metadata: &fs::Metadata) -> bool {
    true
}
