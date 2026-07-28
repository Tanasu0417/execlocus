use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Profile {
    ShareFirst,
    #[default]
    Balanced,
    LinuxFirst,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeKind {
    WindowsNative,
    Wsl,
    LinuxNative,
    #[default]
    Unknown,
}

impl RuntimeKind {
    #[must_use]
    pub const fn native_executable_origin(self) -> ExecutableOrigin {
        match self {
            Self::WindowsNative => ExecutableOrigin::Windows,
            Self::Wsl | Self::LinuxNative => ExecutableOrigin::Linux,
            Self::Unknown => ExecutableOrigin::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Certain,
    High,
    Medium,
    Low,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationStatus {
    Observed,
    Inferred,
    #[default]
    Unavailable,
    Failed,
}

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeInfo {
    pub kind: RuntimeKind,
    pub os_name: String,
    pub distribution: Option<String>,
    pub user: Option<String>,
    pub shell: Option<String>,
    pub terminal: Option<String>,
    pub status: ObservationStatus,
    pub confidence: Confidence,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PathClass {
    WindowsNative,
    WindowsMounted,
    WslNative,
    WslUnc,
    LinuxNative,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProjectInfo {
    pub path: Option<String>,
    pub class: PathClass,
    pub status: ObservationStatus,
    pub confidence: Confidence,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutableFormat {
    Pe,
    Elf,
    Script,
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutableOrigin {
    Windows,
    Linux,
    Script,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExecutableCandidate {
    pub path: String,
    pub format: ExecutableFormat,
    pub origin: ExecutableOrigin,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExecutableInfo {
    pub role: String,
    pub requested: String,
    pub selected: Option<ExecutableCandidate>,
    pub candidates: Vec<ExecutableCandidate>,
    pub status: ObservationStatus,
    pub confidence: Confidence,
}

#[derive(Clone, Debug, Serialize)]
pub struct Evidence {
    pub id: String,
    pub probe: String,
    pub kind: String,
    pub claim: String,
    pub value: Option<String>,
    pub sensitive: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProbeFailure {
    pub probe: String,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub summary: String,
    pub evidence_ids: Vec<String>,
    pub suggested_actions: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TopologyNode {
    pub id: String,
    pub kind: String,
    pub label: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TopologyEdge {
    pub from: String,
    pub relation: String,
    pub to: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Topology {
    pub nodes: Vec<TopologyNode>,
    pub edges: Vec<TopologyEdge>,
}

impl Topology {
    #[must_use]
    pub fn from_report(report: &Report) -> Self {
        let mut topology = Self::default();
        topology.nodes.push(TopologyNode {
            id: "runtime.current".to_owned(),
            kind: "runtime".to_owned(),
            label: format!("{:?}", report.runtime.kind),
        });

        if let Some(path) = &report.project.path {
            topology.nodes.push(TopologyNode {
                id: "project.current".to_owned(),
                kind: "project".to_owned(),
                label: path.clone(),
            });
            topology.edges.push(TopologyEdge {
                from: "runtime.current".to_owned(),
                relation: "works-on".to_owned(),
                to: "project.current".to_owned(),
            });
        }

        for executable in &report.executables {
            if let Some(selected) = &executable.selected {
                let id = format!("executable.{}", executable.role);
                topology.nodes.push(TopologyNode {
                    id: id.clone(),
                    kind: "executable".to_owned(),
                    label: selected.path.clone(),
                });
                topology.edges.push(TopologyEdge {
                    from: "runtime.current".to_owned(),
                    relation: "resolves-to".to_owned(),
                    to: id,
                });
            }
        }

        topology
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub schema_version: String,
    pub generated_at_unix_ms: u128,
    pub profile: Profile,
    pub runtime: RuntimeInfo,
    pub project: ProjectInfo,
    pub executables: Vec<ExecutableInfo>,
    pub topology: Topology,
    pub evidence: Vec<Evidence>,
    pub findings: Vec<Finding>,
    pub probe_failures: Vec<ProbeFailure>,
}

impl Report {
    #[must_use]
    pub fn has_error_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == Severity::Error)
    }
}

pub struct ProbeResult<T> {
    pub value: T,
    pub evidence: Vec<Evidence>,
    pub failures: Vec<ProbeFailure>,
}
