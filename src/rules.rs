use crate::model::{
    AgentStateLocation, Confidence, ExecutableOrigin, Finding, ObservationStatus, PathClass,
    Profile, Report, RuntimeKind, Severity, ToolchainState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub default_severity: &'static str,
    pub rationale: &'static str,
    pub required_evidence: &'static [&'static str],
    pub suggested_actions: &'static [&'static str],
}

pub const RULE_DEFINITIONS: &[RuleDefinition] = &[
    RuleDefinition {
        id: "ENV001",
        title: "Visible terminal and agent runtime differ",
        category: "environment",
        default_severity: "info",
        rationale: "Commands run in the visible terminal can resolve different paths, tools, configuration, and permissions from commands run by the agent.",
        required_evidence: &["terminal layer", "agent runtime", "relationship evidence"],
        suggested_actions: &[
            "Compare resolved Git, Node, shell, and project paths before changing configuration.",
            "Use a structured report to inspect the relationship evidence.",
        ],
    },
    RuleDefinition {
        id: "ENV002",
        title: "WSL execution resolves a Windows executable",
        category: "environment",
        default_severity: "warning",
        rationale: "Path syntax, permissions, subprocess behavior, configuration directories, and package installation targets can differ from Linux-native expectations.",
        required_evidence: &[
            "runtime layer",
            "resolved executable path",
            "executable format or origin",
        ],
        suggested_actions: &[
            "Keep the setup when Windows interoperability is intentional and document the boundary.",
            "If Linux behavior is intended, install and prioritize the Linux-native executable in the distribution.",
            "Inspect PATH001 before changing PATH order.",
        ],
    },
    RuleDefinition {
        id: "ENV003",
        title: "Agent is installed in both Windows and WSL",
        category: "environment",
        default_severity: "info",
        rationale: "The selected installation can vary with the terminal, PATH, launcher, or desktop integration, while configuration and versions can diverge.",
        required_evidence: &["certain agent installation paths in Windows and WSL"],
        suggested_actions: &[
            "Compare versions and resolved paths before changing either installation.",
            "Keep both installations when both workflows are intentional.",
            "Remove or deprioritize one only after confirming the active workflow.",
        ],
    },
    RuleDefinition {
        id: "ENV004",
        title: "Agent state or configuration crosses OS layers",
        category: "environment",
        default_severity: "warning",
        rationale: "File locking, permissions, line endings, performance, and concurrent access can become inconsistent when writable state crosses layers.",
        required_evidence: &[
            "agent runtime",
            "normalized config or state path",
            "path classification",
        ],
        suggested_actions: &[
            "Keep writable databases, caches, and primary configuration native to the executor when practical.",
            "Share only configuration files documented as portable by the agent vendor.",
            "Back up state before manually relocating it.",
        ],
    },
    RuleDefinition {
        id: "FS001",
        title: "Project or heavy artifacts are on a Windows mount",
        category: "filesystem",
        default_severity: "profile-dependent",
        rationale: "Windows-mounted storage improves interoperability but can change metadata-heavy I/O, permissions, symlinks, watchers, or case-sensitivity behavior.",
        required_evidence: &["WSL runtime", "path classification", "selected profile"],
        suggested_actions: &[
            "Keep the Windows mount when Windows application access is the priority.",
            "Measure the affected workload before moving files for performance reasons.",
            "Use the selected profile's finding for profile-specific guidance.",
        ],
    },
    RuleDefinition {
        id: "FS002",
        title: "WSL-native project may be inconvenient for a Windows-first workflow",
        category: "filesystem",
        default_severity: "info",
        rationale: "Windows tools can usually use WSL UNC paths, but some applications, dialogs, watchers, or integrations can be less convenient.",
        required_evidence: &["WSL-native project path", "share-first profile"],
        suggested_actions: &[
            "Keep the WSL-native project when Linux compatibility and filesystem behavior matter more.",
            "Access the project from Windows through its WSL UNC path.",
            "Move it only if a required Windows application cannot use the UNC path reliably.",
        ],
    },
    RuleDefinition {
        id: "PATH001",
        title: "PATH precedence selects an executable from another layer",
        category: "path",
        default_severity: "warning",
        rationale: "The selected version can use different configuration, packages, path rules, or subprocess semantics from the native candidate.",
        required_evidence: &[
            "ordered candidates",
            "selected executable path",
            "runtime layer",
            "candidate origins",
        ],
        suggested_actions: &[
            "Review PATH order for the active shell before changing configuration.",
            "Confirm version and behavior before modifying shell profiles.",
            "Use an explicit executable path in reproducible automation.",
        ],
    },
    RuleDefinition {
        id: "GIT001",
        title: "Git and project reside in different OS layers",
        category: "toolchain",
        default_severity: "warning",
        rationale: "Credentials, file modes, case sensitivity, hooks, line endings, and path handling can differ across OS layers.",
        required_evidence: &["resolved Git origin", "project path classification"],
        suggested_actions: &[
            "Prefer Git native to the runtime that owns the project workflow.",
            "Review line endings, file modes, hooks, and credential helpers before switching.",
        ],
    },
    RuleDefinition {
        id: "TOOL001",
        title: "npm is selected while Node is not found",
        category: "toolchain",
        default_severity: "warning",
        rationale: "npm launchers can be wrapper-specific, while JavaScript tools that invoke node directly still require a resolvable Node command.",
        required_evidence: &[
            "selected npm command",
            "complete shell evidence with Node not found",
        ],
        suggested_actions: &[
            "Run the independent npm and Node verification commands in the same shell.",
            "Keep the wrapper only if both npm and the intended JavaScript workflow succeed.",
            "Otherwise initialize or install Node for that shell before changing unrelated PATH entries.",
        ],
    },
];

#[must_use]
pub fn definition(rule_id: &str) -> Option<&'static RuleDefinition> {
    RULE_DEFINITIONS
        .iter()
        .find(|definition| definition.id.eq_ignore_ascii_case(rule_id))
}

#[must_use]
pub fn evaluate(report: &Report) -> Vec<Finding> {
    let mut findings = Vec::new();
    evaluate_env001(report, &mut findings);
    evaluate_env003(report, &mut findings);
    evaluate_env004(report, &mut findings);
    evaluate_fs001(report, &mut findings);
    evaluate_fs002(report, &mut findings);
    evaluate_env002(report, &mut findings);
    evaluate_path001(report, &mut findings);
    evaluate_git001(report, &mut findings);
    evaluate_tool001(report, &mut findings);
    findings.sort_by(|left, right| {
        right
            .severity
            .cmp(&left.severity)
            .then_with(|| left.id.cmp(&right.id))
    });
    findings
}

fn evaluate_env001(report: &Report, findings: &mut Vec<Finding>) {
    if report.runtime.terminal_layer == RuntimeKind::Unknown
        || !is_available(report.runtime.terminal_layer_status)
        || report.runtime.terminal_layer_confidence != Confidence::Certain
        || report.agent.product.is_none()
        || report.agent.runtime == RuntimeKind::Unknown
        || !is_available(report.agent.runtime_status)
        || !is_high_confidence(report.agent.runtime_confidence)
        || !is_high_confidence(report.agent.product_confidence)
        || report.runtime.terminal_layer == report.agent.runtime
    {
        return;
    }

    findings.push(Finding {
        id: "ENV001".to_owned(),
        title: "Terminal session and agent use different OS layers".to_owned(),
        severity: Severity::Info,
        summary: format!(
            "The active terminal session is {:?}, while the observed agent execution is {:?}.",
            report.runtime.terminal_layer, report.agent.runtime
        ),
        evidence_ids: vec![
            "terminal.layer".to_owned(),
            "agent.product".to_owned(),
            "agent.runtime".to_owned(),
        ],
        suggested_actions: vec![
            "Compare resolved Git, Node, shell, and project paths before changing configuration."
                .to_owned(),
            "Use a structured report to inspect the relationship evidence.".to_owned(),
        ],
        verification_steps: vec![
            "Run the Toolchain verification commands in both the visible terminal and agent execution context, then rerun ExecLocus."
                .to_owned(),
        ],
    });
}

fn evaluate_env003(report: &Report, findings: &mut Vec<Finding>) {
    if report.runtime.kind != RuntimeKind::Wsl
        || report.runtime.status != ObservationStatus::Observed
        || report.runtime.confidence != Confidence::Certain
    {
        return;
    }

    for installation in &report.agent.installations {
        if installation.status != ObservationStatus::Observed
            || installation.confidence != Confidence::Certain
        {
            continue;
        }

        let windows = installation
            .candidates
            .iter()
            .position(|candidate| candidate.origin == ExecutableOrigin::Windows);
        let linux = installation
            .candidates
            .iter()
            .position(|candidate| candidate.origin == ExecutableOrigin::Linux);
        let (Some(windows), Some(linux)) = (windows, linux) else {
            continue;
        };

        findings.push(Finding {
            id: "ENV003".to_owned(),
            title: format!(
                "{} is available in Windows and WSL layers",
                installation.product.label()
            ),
            severity: Severity::Info,
            summary: format!(
                "Certain executable evidence found {} candidates in both Windows and WSL layers.",
                installation.product.label()
            ),
            evidence_ids: vec![
                "runtime.kind".to_owned(),
                format!(
                    "agent.installation.{}.candidate.{}",
                    installation.product.evidence_value(),
                    windows + 1
                ),
                format!(
                    "agent.installation.{}.candidate.{}",
                    installation.product.evidence_value(),
                    linux + 1
                ),
            ],
            suggested_actions: vec![
                "Compare versions and resolved paths before changing either installation."
                    .to_owned(),
                "Keep both installations when both workflows are intentional.".to_owned(),
                "Remove or deprioritize one only after confirming the active workflow.".to_owned(),
            ],
            verification_steps: vec![format!(
                "Run the {} verification command shown in Toolchain from both Windows and WSL, then rerun ExecLocus.",
                installation.product.label()
            )],
        });
    }
}

fn evaluate_env004(report: &Report, findings: &mut Vec<Finding>) {
    if report.agent.product.is_none()
        || report.agent.runtime == RuntimeKind::Unknown
        || !is_available(report.agent.runtime_status)
        || !is_high_confidence(report.agent.runtime_confidence)
        || !is_high_confidence(report.agent.product_confidence)
    {
        return;
    }

    for state in &report.agent.state_locations {
        if Some(state.product) != report.agent.product
            || !is_available(state.status)
            || !is_high_confidence(state.confidence)
            || !state_crosses_runtime(report.agent.runtime, state)
        {
            continue;
        }

        findings.push(Finding {
            id: "ENV004".to_owned(),
            title: format!("{} configuration crosses OS layers", state.product.label()),
            severity: Severity::Warning,
            summary: format!(
                "The active agent runs in {:?}, while its {} root is classified as {:?}.",
                report.agent.runtime,
                state.kind.label(),
                state.class
            ),
            evidence_ids: vec![
                "agent.product".to_owned(),
                "agent.runtime".to_owned(),
                state.evidence_id(),
            ],
            suggested_actions: vec![
                "Keep writable databases, caches, and primary configuration native to the executor when practical."
                    .to_owned(),
                "Share only configuration files documented as portable by the agent vendor."
                    .to_owned(),
                "Back up state before manually relocating it.".to_owned(),
            ],
            verification_steps: vec![
                "Rerun ExecLocus after changing the configuration location and confirm ENV004 is absent."
                    .to_owned(),
            ],
        });
    }
}

fn is_available(status: ObservationStatus) -> bool {
    matches!(
        status,
        ObservationStatus::Observed | ObservationStatus::Inferred
    )
}

fn is_high_confidence(confidence: Confidence) -> bool {
    matches!(confidence, Confidence::High | Confidence::Certain)
}

fn state_crosses_runtime(runtime: RuntimeKind, state: &AgentStateLocation) -> bool {
    matches!(
        (runtime, state.class),
        (
            RuntimeKind::Wsl,
            PathClass::WindowsNative | PathClass::WindowsMounted
        ) | (
            RuntimeKind::WindowsNative,
            PathClass::WslNative | PathClass::WslUnc
        )
    )
}

fn evaluate_fs001(report: &Report, findings: &mut Vec<Finding>) {
    if report.runtime.kind != RuntimeKind::Wsl
        || report.runtime.status != ObservationStatus::Observed
        || report.runtime.confidence != Confidence::Certain
        || report.project.class != PathClass::WindowsMounted
        || !has_certain_project_evidence(report)
    {
        return;
    }

    let (severity, suggested_actions) = match report.profile {
        Profile::ShareFirst => (
            Severity::Info,
            vec![
                "Keep the Windows-mounted project when Windows apps, Explorer, or Cowork access is the priority."
                    .to_owned(),
                "Measure the affected workload before moving files for performance reasons."
                    .to_owned(),
            ],
        ),
        Profile::Balanced => (
            Severity::Info,
            vec![
                "Keep shared source on the Windows mount when interoperability is useful."
                    .to_owned(),
                "Where supported, place dependency caches and build output in WSL-native storage."
                    .to_owned(),
            ],
        ),
        Profile::LinuxFirst => (
            Severity::Warning,
            vec![
                r"Consider a WSL-native checkout and access it from Windows through \\wsl.localhost."
                    .to_owned(),
                "Confirm that required Windows applications work with the WSL UNC path before moving."
                    .to_owned(),
            ],
        ),
    };

    findings.push(Finding {
        id: "FS001".to_owned(),
        title: "WSL project uses a Windows-mounted path".to_owned(),
        severity,
        summary: "The project is stored on a Windows filesystem mounted into WSL. This is a supported interoperability choice with filesystem tradeoffs."
            .to_owned(),
        evidence_ids: vec![
            "runtime.kind".to_owned(),
            "project.path".to_owned(),
            "profile.selected".to_owned(),
        ],
        suggested_actions,
        verification_steps: vec![
            "Rerun ExecLocus with the same profile after changing project or cache placement."
                .to_owned(),
        ],
    });
}

fn evaluate_fs002(report: &Report, findings: &mut Vec<Finding>) {
    if report.profile != Profile::ShareFirst
        || report.project.class != PathClass::WslNative
        || !has_certain_project_evidence(report)
    {
        return;
    }

    findings.push(Finding {
        id: "FS002".to_owned(),
        title: "Share-first project is stored in WSL-native storage".to_owned(),
        severity: Severity::Info,
        summary: "The project is WSL-native while the selected profile prioritizes convenient Windows sharing."
            .to_owned(),
        evidence_ids: vec!["project.path".to_owned(), "profile.selected".to_owned()],
        suggested_actions: vec![
            "Keep the WSL-native project when Linux compatibility and filesystem behavior matter more."
                .to_owned(),
            r"Access the project from Windows through its \\wsl.localhost UNC path.".to_owned(),
            "Move it only if a required Windows application cannot use the UNC path reliably."
                .to_owned(),
        ],
        verification_steps: vec![
            "Rerun ExecLocus with the share-first profile after changing the project location."
                .to_owned(),
        ],
    });
}

fn has_certain_project_evidence(report: &Report) -> bool {
    report.project.path.is_some()
        && report.project.status == ObservationStatus::Observed
        && report.project.confidence == Confidence::Certain
}

fn evaluate_env002(report: &Report, findings: &mut Vec<Finding>) {
    if report.runtime.kind != RuntimeKind::Wsl {
        return;
    }

    for executable in &report.executables {
        let Some(selected) = &executable.selected else {
            continue;
        };
        if selected.origin != ExecutableOrigin::Windows {
            continue;
        }

        findings.push(Finding {
            id: "ENV002".to_owned(),
            title: format!("WSL execution resolves Windows {}", executable.role),
            severity: Severity::Warning,
            summary: format!(
                "{} resolves to a Windows executable while ExecLocus runs in WSL.",
                executable.role
            ),
            evidence_ids: vec![
                "runtime.kind".to_owned(),
                format!("executable.{}", executable.role),
            ],
            suggested_actions: vec![
                "Keep the setup if Windows interoperability is intentional.".to_owned(),
                format!(
                    "If Linux behavior is intended, install and prioritize Linux-native {}.",
                    executable.role
                ),
            ],
            verification_steps: vec![format!(
                "Run `{}` in the same shell, then rerun ExecLocus.",
                executable.verification_command
            )],
        });
    }
}

fn evaluate_path001(report: &Report, findings: &mut Vec<Finding>) {
    let native_origin = report.runtime.kind.native_executable_origin();
    if native_origin == ExecutableOrigin::Unknown {
        return;
    }

    for executable in &report.executables {
        let Some(selected) = &executable.selected else {
            continue;
        };
        if selected.origin == native_origin
            || !matches!(
                selected.origin,
                ExecutableOrigin::Windows | ExecutableOrigin::Linux
            )
        {
            continue;
        }
        if !executable
            .candidates
            .iter()
            .skip(1)
            .any(|candidate| candidate.origin == native_origin)
        {
            continue;
        }

        findings.push(Finding {
            id: "PATH001".to_owned(),
            title: format!("PATH selects cross-layer {}", executable.role),
            severity: Severity::Warning,
            summary: format!(
                "PATH selects {} even though a native candidate is also available.",
                selected.path
            ),
            evidence_ids: vec![format!("executable.{}", executable.role)],
            suggested_actions: vec![
                "Review PATH order for the active shell before changing configuration.".to_owned(),
                "Use an explicit executable path in reproducible automation.".to_owned(),
            ],
            verification_steps: vec![format!(
                "Run `{}` in the same shell, then rerun ExecLocus.",
                executable.verification_command
            )],
        });
    }
}

fn evaluate_git001(report: &Report, findings: &mut Vec<Finding>) {
    let Some(git_info) = report
        .executables
        .iter()
        .find(|executable| executable.role == "git")
    else {
        return;
    };
    let Some(git) = git_info.selected.as_ref() else {
        return;
    };

    let conflict = matches!(
        (report.project.class, git.origin),
        (PathClass::WslNative, ExecutableOrigin::Windows)
            | (PathClass::WindowsNative, ExecutableOrigin::Linux)
    );
    if !conflict {
        return;
    }

    findings.push(Finding {
        id: "GIT001".to_owned(),
        title: "Git and project use different OS layers".to_owned(),
        severity: Severity::Warning,
        summary: format!(
            "Git resolves to {:?} while the project is classified as {:?}.",
            git.origin, report.project.class
        ),
        evidence_ids: vec!["project.path".to_owned(), "executable.git".to_owned()],
        suggested_actions: vec![
            "Prefer Git native to the runtime that owns the project workflow.".to_owned(),
            "Review line endings, file modes, hooks, and credential helpers before switching."
                .to_owned(),
        ],
        verification_steps: vec![format!(
            "Run `{}` in the same shell, then rerun ExecLocus.",
            git_info.verification_command
        )],
    });
}

fn evaluate_tool001(report: &Report, findings: &mut Vec<Finding>) {
    let Some(npm) = report
        .executables
        .iter()
        .find(|executable| executable.role == "npm")
    else {
        return;
    };
    let Some(node) = report
        .executables
        .iter()
        .find(|executable| executable.role == "node")
    else {
        return;
    };
    if npm.selection_state != ToolchainState::Selected
        || node.selection_state != ToolchainState::NotFound
    {
        return;
    }

    findings.push(Finding {
        id: "TOOL001".to_owned(),
        title: "npm is selected while Node is not found".to_owned(),
        severity: Severity::Warning,
        summary: "npm resolves in this shell, but Node does not. Direct Node commands and JavaScript tools that require node on PATH may fail even if this npm wrapper starts."
            .to_owned(),
        evidence_ids: vec![
            "executable.npm".to_owned(),
            "executable.node.resolution".to_owned(),
        ],
        suggested_actions: vec![
            "Run the independent npm and Node verification commands in the same shell."
                .to_owned(),
            "Keep the wrapper only if both npm and the intended JavaScript workflow succeed."
                .to_owned(),
            "Otherwise initialize or install Node for that shell before changing unrelated PATH entries."
                .to_owned(),
        ],
        verification_steps: vec![format!(
            "Run `{}` and `{}`, then rerun ExecLocus.",
            npm.verification_command, node.verification_command
        )],
    });
}

#[cfg(test)]
mod tests {
    use crate::model::{
        AgentInfo, AgentInstallationInfo, AgentProduct, AgentStateKind, AgentStateLocation,
        Confidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo, ExecutableOrigin,
        ExecutableResolutionMethod, ExecutableSelectionKind, ObservationStatus, PathClass, Profile,
        ProjectInfo, Report, RuntimeInfo, RuntimeKind, Severity, ToolchainState, Topology,
    };

    use super::{RULE_DEFINITIONS, definition, evaluate};

    #[test]
    fn rule_catalog_is_unique_complete_and_case_insensitive() {
        let ids = RULE_DEFINITIONS
            .iter()
            .map(|definition| definition.id)
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(ids.len(), RULE_DEFINITIONS.len());
        assert_eq!(
            ids,
            std::collections::HashSet::from([
                "ENV001", "ENV002", "ENV003", "ENV004", "FS001", "FS002", "PATH001", "GIT001",
                "TOOL001",
            ])
        );
        assert_eq!(definition("env002").map(|item| item.id), Some("ENV002"));
        assert!(definition("UNKNOWN").is_none());
    }

    fn report(
        runtime_kind: RuntimeKind,
        project_class: PathClass,
        executables: Vec<ExecutableInfo>,
    ) -> Report {
        report_with_profile(runtime_kind, project_class, executables, Profile::Balanced)
    }

    fn report_with_profile(
        runtime_kind: RuntimeKind,
        project_class: PathClass,
        executables: Vec<ExecutableInfo>,
        profile: Profile,
    ) -> Report {
        Report {
            schema_version: "test".to_owned(),
            generated_at_unix_ms: 0,
            profile,
            runtime: RuntimeInfo {
                kind: runtime_kind,
                kind_source: None,
                os_name: "test".to_owned(),
                distribution: None,
                distribution_source: None,
                user: None,
                user_source: None,
                shell: None,
                shell_source: None,
                terminal: None,
                terminal_layer: RuntimeKind::Unknown,
                terminal_layer_status: ObservationStatus::Unavailable,
                terminal_layer_confidence: Confidence::None,
                terminal_layer_source: None,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            agent: AgentInfo::default(),
            project: ProjectInfo {
                path: Some("test".to_owned()),
                class: project_class,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            executables,
            topology: Topology::default(),
            evidence: Vec::new(),
            findings: Vec::new(),
            probe_failures: Vec::new(),
        }
    }

    fn observed_agent(product: AgentProduct, runtime: RuntimeKind) -> AgentInfo {
        AgentInfo {
            product: Some(product),
            product_status: ObservationStatus::Inferred,
            product_confidence: Confidence::High,
            product_source: None,
            runtime,
            runtime_status: ObservationStatus::Observed,
            runtime_confidence: Confidence::Certain,
            installations: Vec::new(),
            state_locations: Vec::new(),
        }
    }

    #[test]
    fn env001_requires_certain_terminal_layer_and_high_confidence_agent_runtime() {
        let mut mismatch = report(RuntimeKind::Wsl, PathClass::WslNative, Vec::new());
        mismatch.runtime.terminal_layer = RuntimeKind::WindowsNative;
        mismatch.runtime.terminal_layer_status = ObservationStatus::Inferred;
        mismatch.runtime.terminal_layer_confidence = Confidence::Certain;
        mismatch.agent = observed_agent(AgentProduct::Codex, RuntimeKind::Wsl);
        assert!(evaluate(&mismatch).iter().any(|item| item.id == "ENV001"));

        mismatch.runtime.terminal_layer = RuntimeKind::Wsl;
        assert!(!evaluate(&mismatch).iter().any(|item| item.id == "ENV001"));

        mismatch.runtime.terminal_layer = RuntimeKind::WindowsNative;
        mismatch.runtime.terminal_layer_confidence = Confidence::High;
        assert!(!evaluate(&mismatch).iter().any(|item| item.id == "ENV001"));
    }

    #[test]
    fn env003_requires_certain_candidates_in_both_layers() {
        let mut report = report(RuntimeKind::Wsl, PathClass::WslNative, Vec::new());
        report.agent.installations.push(AgentInstallationInfo {
            product: AgentProduct::Codex,
            candidates: executable(
                "codex",
                ExecutableOrigin::Linux,
                &[ExecutableOrigin::Windows],
            )
            .candidates,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        });
        let both = evaluate(&report);
        let finding = both
            .iter()
            .find(|item| item.id == "ENV003")
            .expect("two certain agent candidates should be explained");
        assert_eq!(finding.severity, Severity::Info);
        assert_eq!(finding.evidence_ids.len(), 3);

        report.agent.installations[0].candidates =
            executable("codex", ExecutableOrigin::Linux, &[]).candidates;
        let one_layer = evaluate(&report);
        assert!(!one_layer.iter().any(|item| item.id == "ENV003"));

        report.agent.installations[0].candidates = executable(
            "codex",
            ExecutableOrigin::Linux,
            &[ExecutableOrigin::Windows],
        )
        .candidates;
        report.runtime.kind = RuntimeKind::LinuxNative;
        assert!(!evaluate(&report).iter().any(|item| item.id == "ENV003"));
    }

    #[test]
    fn env004_requires_high_confidence_cross_layer_primary_state() {
        let mut crossing = report(RuntimeKind::Wsl, PathClass::WslNative, Vec::new());
        crossing.agent = observed_agent(AgentProduct::ClaudeCode, RuntimeKind::Wsl);
        crossing.agent.state_locations.push(AgentStateLocation {
            product: AgentProduct::ClaudeCode,
            kind: AgentStateKind::PrimaryConfig,
            path: "/mnt/c/demo/.claude".to_owned(),
            class: PathClass::WindowsMounted,
            status: ObservationStatus::Inferred,
            confidence: Confidence::High,
        });
        assert!(evaluate(&crossing).iter().any(|item| item.id == "ENV004"));

        crossing.agent.state_locations[0].class = PathClass::WslNative;
        assert!(!evaluate(&crossing).iter().any(|item| item.id == "ENV004"));

        crossing.agent.state_locations[0].class = PathClass::WindowsMounted;
        crossing.agent.state_locations[0].confidence = Confidence::Medium;
        assert!(!evaluate(&crossing).iter().any(|item| item.id == "ENV004"));
    }

    #[test]
    fn fs001_changes_advice_and_severity_by_profile() {
        let share_first = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::ShareFirst,
        ));
        let balanced = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::Balanced,
        ));
        let linux_first = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::LinuxFirst,
        ));

        let share_first = share_first
            .iter()
            .find(|finding| finding.id == "FS001")
            .expect("share-first should explain the mounted project");
        let balanced = balanced
            .iter()
            .find(|finding| finding.id == "FS001")
            .expect("balanced should explain the mounted project");
        let linux_first = linux_first
            .iter()
            .find(|finding| finding.id == "FS001")
            .expect("linux-first should explain the mounted project");

        assert_eq!(share_first.severity, Severity::Info);
        assert_eq!(balanced.severity, Severity::Info);
        assert_eq!(linux_first.severity, Severity::Warning);
        assert_ne!(share_first.suggested_actions, balanced.suggested_actions);
        assert_ne!(balanced.suggested_actions, linux_first.suggested_actions);
        assert!(
            balanced
                .summary
                .contains("supported interoperability choice")
        );
    }

    #[test]
    fn fs001_requires_observed_certain_runtime_and_project_evidence() {
        let mut missing_path = report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::LinuxFirst,
        );
        missing_path.project.path = None;
        assert!(
            !evaluate(&missing_path)
                .iter()
                .any(|item| item.id == "FS001")
        );

        let mut inferred_runtime = report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::LinuxFirst,
        );
        inferred_runtime.runtime.status = ObservationStatus::Inferred;
        inferred_runtime.runtime.confidence = Confidence::High;
        assert!(
            !evaluate(&inferred_runtime)
                .iter()
                .any(|item| item.id == "FS001")
        );
    }

    #[test]
    fn fs001_does_not_trigger_outside_wsl_or_for_wsl_native_projects() {
        let linux = evaluate(&report_with_profile(
            RuntimeKind::LinuxNative,
            PathClass::WindowsMounted,
            Vec::new(),
            Profile::LinuxFirst,
        ));
        let wsl_native = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            Vec::new(),
            Profile::LinuxFirst,
        ));
        assert!(!linux.iter().any(|item| item.id == "FS001"));
        assert!(!wsl_native.iter().any(|item| item.id == "FS001"));
    }

    #[test]
    fn fs002_only_explains_wsl_native_projects_in_share_first() {
        let share_first = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            Vec::new(),
            Profile::ShareFirst,
        ));
        let balanced = evaluate(&report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            Vec::new(),
            Profile::Balanced,
        ));

        let finding = share_first
            .iter()
            .find(|item| item.id == "FS002")
            .expect("share-first should explain the WSL-native project");
        assert_eq!(finding.severity, Severity::Info);
        assert!(!balanced.iter().any(|item| item.id == "FS002"));

        let mut missing_path = report_with_profile(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            Vec::new(),
            Profile::ShareFirst,
        );
        missing_path.project.path = None;
        assert!(
            !evaluate(&missing_path)
                .iter()
                .any(|item| item.id == "FS002")
        );
    }

    fn executable(
        role: &str,
        selected_origin: ExecutableOrigin,
        additional_origins: &[ExecutableOrigin],
    ) -> ExecutableInfo {
        let candidate = |origin| ExecutableCandidate {
            path: format!("{origin:?}-{role}"),
            format: match origin {
                ExecutableOrigin::Windows => ExecutableFormat::Pe,
                ExecutableOrigin::Linux => ExecutableFormat::Elf,
                _ => ExecutableFormat::Unknown,
            },
            origin,
        };
        let mut candidates = vec![candidate(selected_origin)];
        candidates.extend(additional_origins.iter().copied().map(candidate));
        ExecutableInfo {
            role: role.to_owned(),
            requested: role.to_owned(),
            selection_state: ToolchainState::Selected,
            selected: candidates.first().cloned(),
            selected_kind: Some(ExecutableSelectionKind::Application),
            selected_binding: None,
            candidates,
            resolution_method: ExecutableResolutionMethod::PathFallback,
            resolution_shell: None,
            shell_session_complete: None,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
            selection_reason: "PATH order selected this candidate.".to_owned(),
            verification_command: format!("type -a -- {role}"),
        }
    }

    #[test]
    fn env002_requires_wsl_and_windows_executable() {
        let findings = evaluate(&report(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            vec![executable("node", ExecutableOrigin::Windows, &[])],
        ));
        assert!(findings.iter().any(|finding| finding.id == "ENV002"));

        let findings = evaluate(&report(
            RuntimeKind::LinuxNative,
            PathClass::LinuxNative,
            vec![executable("node", ExecutableOrigin::Windows, &[])],
        ));
        assert!(!findings.iter().any(|finding| finding.id == "ENV002"));
    }

    #[test]
    fn path001_requires_a_native_alternative() {
        let findings = evaluate(&report(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            vec![executable(
                "node",
                ExecutableOrigin::Windows,
                &[ExecutableOrigin::Linux],
            )],
        ));
        assert!(findings.iter().any(|finding| finding.id == "PATH001"));
    }

    #[test]
    fn tool001_requires_selected_npm_and_certainly_missing_node() {
        let npm = executable("npm", ExecutableOrigin::Linux, &[]);
        let mut node = executable("node", ExecutableOrigin::Linux, &[]);
        node.selection_state = ToolchainState::NotFound;
        node.selected = None;
        node.selected_kind = None;
        node.candidates.clear();

        let missing_node_report = report(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            vec![npm, node.clone()],
        );
        let finding = evaluate(&missing_node_report)
            .into_iter()
            .find(|item| item.id == "TOOL001")
            .expect("missing Node should be actionable when npm is selected");
        assert_eq!(finding.severity, Severity::Warning);
        assert!(finding.summary.contains("Direct Node commands"));
        assert!(!finding.verification_steps.is_empty());

        node.selection_state = ToolchainState::CandidatesUnconfirmed;
        node.candidates.push(ExecutableCandidate {
            path: "/usr/bin/node".to_owned(),
            format: ExecutableFormat::Elf,
            origin: ExecutableOrigin::Linux,
        });
        let uncertain = report(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            vec![executable("npm", ExecutableOrigin::Linux, &[]), node],
        );
        assert!(!evaluate(&uncertain).iter().any(|item| item.id == "TOOL001"));
    }

    #[test]
    fn path001_does_not_treat_a_portable_script_as_an_os_layer() {
        let findings = evaluate(&report(
            RuntimeKind::WindowsNative,
            PathClass::WindowsNative,
            vec![executable(
                "npm",
                ExecutableOrigin::Script,
                &[ExecutableOrigin::Windows],
            )],
        ));
        assert!(!findings.iter().any(|finding| finding.id == "PATH001"));
    }

    #[test]
    fn git001_does_not_reject_mnt_c_by_itself() {
        let findings = evaluate(&report(
            RuntimeKind::Wsl,
            PathClass::WindowsMounted,
            vec![executable("git", ExecutableOrigin::Linux, &[])],
        ));
        assert!(!findings.iter().any(|finding| finding.id == "GIT001"));
    }

    #[test]
    fn git001_detects_windows_git_on_wsl_native_project() {
        let findings = evaluate(&report(
            RuntimeKind::Wsl,
            PathClass::WslNative,
            vec![executable("git", ExecutableOrigin::Windows, &[])],
        ));
        assert!(findings.iter().any(|finding| finding.id == "GIT001"));
    }
}
