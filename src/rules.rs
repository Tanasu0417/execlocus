use crate::model::{
    Confidence, ExecutableOrigin, Finding, ObservationStatus, PathClass, Profile, Report,
    RuntimeKind, Severity,
};

#[must_use]
pub fn evaluate(report: &Report) -> Vec<Finding> {
    let mut findings = Vec::new();
    evaluate_fs001(report, &mut findings);
    evaluate_fs002(report, &mut findings);
    evaluate_env002(report, &mut findings);
    evaluate_path001(report, &mut findings);
    evaluate_git001(report, &mut findings);
    findings.sort_by(|left, right| {
        right
            .severity
            .cmp(&left.severity)
            .then_with(|| left.id.cmp(&right.id))
    });
    findings
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
        });
    }
}

fn evaluate_git001(report: &Report, findings: &mut Vec<Finding>) {
    let Some(git) = report
        .executables
        .iter()
        .find(|executable| executable.role == "git")
        .and_then(|executable| executable.selected.as_ref())
    else {
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
    });
}

#[cfg(test)]
mod tests {
    use crate::model::{
        AgentInfo, Confidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo,
        ExecutableOrigin, ObservationStatus, PathClass, Profile, ProjectInfo, Report, RuntimeInfo,
        RuntimeKind, Severity, Topology,
    };

    use super::evaluate;

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
            selected: candidates.first().cloned(),
            candidates,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
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
