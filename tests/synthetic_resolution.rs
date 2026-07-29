use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use execlocus::{
    collect_report_with, collect_report_with_resolver,
    model::{
        Evidence, ExecutableCandidate, ExecutableFormat, ExecutableOrigin, ObservationStatus,
        PathClass, ProbeResult, Profile, RuntimeKind, RuntimeValueSource,
    },
    probes::context::{CandidateSnapshot, HostPlatform, ProbeContext},
    probes::executable::ExecutableResolver,
};

struct FixtureProbeContext {
    host_platform: HostPlatform,
    environment: HashMap<String, String>,
    path_entries: Vec<PathBuf>,
    current_dir: PathBuf,
    candidates: HashMap<(String, String), CandidateSnapshot>,
    candidate_errors: HashMap<(String, String), io::ErrorKind>,
    text_files: HashMap<String, String>,
    now_unix_ms: u128,
}

impl FixtureProbeContext {
    fn wsl() -> Self {
        let environment = HashMap::from([
            ("WSL_DISTRO_NAME".to_owned(), "Ubuntu-24.04".to_owned()),
            ("USER".to_owned(), "demo".to_owned()),
            ("SHELL".to_owned(), "/bin/bash".to_owned()),
            ("TERM".to_owned(), "xterm-256color".to_owned()),
        ]);
        let path_entries = vec![PathBuf::from("/opt/demo-bin"), PathBuf::from("/usr/bin")];
        let candidates = HashMap::from([
            (
                ("/opt/demo-bin".to_owned(), "node".to_owned()),
                CandidateSnapshot::new(
                    PathBuf::from("/opt/demo-bin/node"),
                    true,
                    b"\x7fELFsynthetic-node-primary".to_vec(),
                ),
            ),
            (
                ("/usr/bin".to_owned(), "node".to_owned()),
                CandidateSnapshot::new(
                    PathBuf::from("/usr/bin/node"),
                    true,
                    b"\x7fELFsynthetic-node-alternative".to_vec(),
                ),
            ),
        ]);

        Self {
            host_platform: HostPlatform::Linux,
            environment,
            path_entries,
            current_dir: PathBuf::from("/mnt/c/demo/execlocus-sample"),
            candidates,
            candidate_errors: HashMap::new(),
            text_files: HashMap::from([(
                "/proc/sys/kernel/osrelease".to_owned(),
                "6.6.87.2-microsoft-standard-WSL2".to_owned(),
            )]),
            now_unix_ms: 1_234_567,
        }
    }

    fn windows() -> Self {
        Self {
            host_platform: HostPlatform::Windows,
            environment: HashMap::from([
                ("USERNAME".to_owned(), "demo".to_owned()),
                (
                    "ComSpec".to_owned(),
                    r"C:\Windows\System32\cmd.exe".to_owned(),
                ),
                ("PATHEXT".to_owned(), ".EXE;.CMD".to_owned()),
            ]),
            path_entries: vec![PathBuf::from(r"C:\Tools\node")],
            current_dir: PathBuf::from(r"C:\demo\execlocus-sample"),
            candidates: HashMap::from([(
                (r"C:\Tools\node".to_owned(), "node.exe".to_owned()),
                CandidateSnapshot::new(
                    PathBuf::from(r"C:\Tools\node\node.exe"),
                    true,
                    b"MZsynthetic-node".to_vec(),
                ),
            )]),
            candidate_errors: HashMap::new(),
            text_files: HashMap::new(),
            now_unix_ms: 7_654_321,
        }
    }
}

impl ProbeContext for FixtureProbeContext {
    fn host_platform(&self) -> HostPlatform {
        self.host_platform
    }

    fn os_name(&self) -> String {
        "linux".to_owned()
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
        if let Some(kind) = self.candidate_errors.get(&key) {
            return Err(io::Error::new(*kind, "synthetic inspection failure"));
        }

        Ok(self.candidates.get(&key).cloned().map(|mut snapshot| {
            snapshot.prefix.truncate(prefix_limit);
            snapshot
        }))
    }

    fn read_text(&self, path: &Path, max_bytes: usize) -> io::Result<String> {
        let value = self
            .text_files
            .get(path.to_string_lossy().as_ref())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing fixture"))?;
        let prefix = &value.as_bytes()[..value.len().min(max_bytes)];
        Ok(String::from_utf8_lossy(prefix).into_owned())
    }

    fn now_unix_ms(&self) -> u128 {
        self.now_unix_ms
    }
}

struct FixtureResolver;

impl ExecutableResolver for FixtureResolver {
    fn resolve(
        &self,
        _context: &dyn ProbeContext,
        command: &str,
        _runtime: RuntimeKind,
    ) -> ProbeResult<Vec<ExecutableCandidate>> {
        let candidates = if command == "node" {
            vec![ExecutableCandidate {
                path: "/resolver/node".to_owned(),
                format: ExecutableFormat::Elf,
                origin: ExecutableOrigin::Linux,
            }]
        } else {
            Vec::new()
        };

        ProbeResult {
            value: candidates,
            evidence: vec![Evidence {
                id: format!("resolver.{command}"),
                probe: "fixture-resolver/v1".to_owned(),
                kind: "resolver".to_owned(),
                claim: "fixture resolver was used".to_owned(),
                value: None,
                sensitive: false,
            }],
            failures: Vec::new(),
        }
    }
}

#[test]
fn collects_a_deterministic_wsl_report_without_process_globals() {
    let report = collect_report_with(&FixtureProbeContext::wsl(), Profile::Balanced);

    assert_eq!(report.generated_at_unix_ms, 1_234_567);
    assert_eq!(report.schema_version, "0.3.0");
    assert_eq!(report.runtime.kind, RuntimeKind::Wsl);
    assert_eq!(report.agent.product, None);
    assert_eq!(
        report.runtime.kind_source,
        Some(RuntimeValueSource::KernelRelease)
    );
    assert_eq!(report.runtime.status, ObservationStatus::Observed);
    assert_eq!(report.runtime.distribution.as_deref(), Some("Ubuntu-24.04"));
    assert_eq!(report.runtime.user.as_deref(), Some("demo"));
    assert_eq!(
        report.runtime.user_source,
        Some(RuntimeValueSource::Environment)
    );
    assert_eq!(report.runtime.shell.as_deref(), Some("/bin/bash"));
    assert_eq!(
        report.runtime.shell_source,
        Some(RuntimeValueSource::Environment)
    );
    assert_eq!(report.project.class, PathClass::WindowsMounted);
    assert_eq!(
        report.project.path.as_deref(),
        Some("/mnt/c/demo/execlocus-sample")
    );

    let node = report
        .executables
        .iter()
        .find(|executable| executable.role == "node")
        .expect("the standard node probe should be present");
    assert_eq!(node.status, ObservationStatus::Observed);
    assert_eq!(node.candidates.len(), 2);
    let selected = node.selected.as_ref().expect("node should resolve");
    assert_eq!(selected.path, "/opt/demo-bin/node");
    assert_eq!(selected.format, ExecutableFormat::Elf);
    assert_eq!(selected.origin, ExecutableOrigin::Linux);
    assert_eq!(node.candidates[1].path, "/usr/bin/node");

    assert!(
        report
            .evidence
            .iter()
            .any(|item| item.id == "executable.node")
    );
    assert!(report.topology.edges.iter().any(|edge| {
        edge.from == "runtime.current"
            && edge.relation == "resolves-to"
            && edge.to == "executable.node"
    }));
    let fs001 = report
        .findings
        .iter()
        .find(|finding| finding.id == "FS001")
        .expect("balanced WSL report should explain the Windows-mounted project");
    assert_eq!(fs001.severity, execlocus::model::Severity::Info);
    assert!(
        report
            .evidence
            .iter()
            .any(|item| item.id == "profile.selected" && item.value.as_deref() == Some("balanced"))
    );
}

#[test]
fn applies_windows_pathext_with_injected_platform_semantics() {
    let report = collect_report_with(&FixtureProbeContext::windows(), Profile::Balanced);

    assert_eq!(report.generated_at_unix_ms, 7_654_321);
    assert_eq!(report.runtime.kind, RuntimeKind::WindowsNative);
    assert_eq!(
        report.runtime.kind_source,
        Some(RuntimeValueSource::TargetPlatform)
    );
    assert_eq!(report.project.class, PathClass::WindowsNative);

    let node = report
        .executables
        .iter()
        .find(|executable| executable.role == "node")
        .expect("the standard node probe should be present");
    let selected = node.selected.as_ref().expect("node.exe should resolve");
    assert_eq!(selected.path, r"C:\Tools\node\node.exe");
    assert_eq!(selected.format, ExecutableFormat::Pe);
    assert_eq!(selected.origin, ExecutableOrigin::Windows);
}

#[test]
fn records_candidate_inspection_failures_without_claiming_success() {
    let mut context = FixtureProbeContext::wsl();
    context.candidates.clear();
    context.candidate_errors.insert(
        ("/opt/demo-bin".to_owned(), "node".to_owned()),
        io::ErrorKind::PermissionDenied,
    );

    let report = collect_report_with(&context, Profile::Balanced);
    let node = report
        .executables
        .iter()
        .find(|executable| executable.role == "node")
        .expect("the standard node probe should be present");

    assert_eq!(node.status, ObservationStatus::Failed);
    assert!(node.selected.is_none());
    assert!(report.probe_failures.iter().any(|failure| {
        failure.probe == "executable/v1" && failure.code == "CANDIDATE_INSPECTION_FAILED"
    }));
}

#[test]
fn accepts_an_injected_executable_resolver() {
    let context = FixtureProbeContext::wsl();
    let report = collect_report_with_resolver(&context, &FixtureResolver, Profile::Balanced);
    let node = report
        .executables
        .iter()
        .find(|executable| executable.role == "node")
        .expect("the standard node probe should be present");

    assert_eq!(
        node.selected
            .as_ref()
            .map(|candidate| candidate.path.as_str()),
        Some("/resolver/node")
    );
    assert!(
        report
            .evidence
            .iter()
            .any(|evidence| evidence.id == "resolver.node")
    );
}
