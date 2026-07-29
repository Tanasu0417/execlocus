use crate::model::{ExecutableOrigin, Finding, PathClass, Report, RuntimeKind, Severity};

#[must_use]
pub fn evaluate(report: &Report) -> Vec<Finding> {
    let mut findings = Vec::new();
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
        if selected.origin == native_origin || selected.origin == ExecutableOrigin::Unknown {
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
        RuntimeKind, Topology,
    };

    use super::evaluate;

    fn report(
        runtime_kind: RuntimeKind,
        project_class: PathClass,
        executables: Vec<ExecutableInfo>,
    ) -> Report {
        Report {
            schema_version: "test".to_owned(),
            generated_at_unix_ms: 0,
            profile: Profile::Balanced,
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
