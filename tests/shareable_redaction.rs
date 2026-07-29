use std::collections::HashMap;

use execlocus::{
    model::{
        AgentEvidenceSource, AgentInfo, AgentProduct, Confidence, Evidence, ExecutableCandidate,
        ExecutableFormat, ExecutableInfo, ExecutableOrigin, Finding, ObservationStatus, PathClass,
        ProbeFailure, Profile, ProjectInfo, Report, RuntimeInfo, RuntimeKind, RuntimeValueSource,
        Severity, Topology, TopologyNode,
    },
    privacy::{RedactionContext, redact_with_context},
    renderers::{json, markdown},
};

struct FixtureRedactionContext {
    environment: HashMap<String, String>,
}

impl RedactionContext for FixtureRedactionContext {
    fn env_var(&self, key: &str) -> Option<String> {
        self.environment.get(key).cloned()
    }
}

fn private_fixture() -> (Report, FixtureRedactionContext) {
    let private_project = "/mnt/c/Users/Alice/secret-project";
    let windows_git = r"C:\Users\Alice\AppData\Local\Programs\Git\cmd\git.exe";
    let linux_git = "/usr/bin/git";
    let report = Report {
        schema_version: "0.3.0".to_owned(),
        generated_at_unix_ms: 123,
        profile: Profile::Balanced,
        runtime: RuntimeInfo {
            kind: RuntimeKind::Wsl,
            kind_source: Some(RuntimeValueSource::KernelRelease),
            os_name: "WSL on WORKSTATION-42".to_owned(),
            distribution: Some("Ubuntu-24.04".to_owned()),
            distribution_source: Some(RuntimeValueSource::Environment),
            user: Some("Alice".to_owned()),
            user_source: Some(RuntimeValueSource::OsAccount),
            shell: Some(r"C:\Users\Alice\bin\bash.exe".to_owned()),
            shell_source: Some(RuntimeValueSource::Environment),
            terminal: Some("WORKSTATION-42 terminal".to_owned()),
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        },
        agent: AgentInfo {
            product: Some(AgentProduct::Codex),
            product_status: ObservationStatus::Inferred,
            product_confidence: Confidence::High,
            product_source: Some(AgentEvidenceSource::ProcessAncestry),
            runtime: RuntimeKind::Wsl,
            runtime_status: ObservationStatus::Observed,
            runtime_confidence: Confidence::Certain,
        },
        project: ProjectInfo {
            path: Some(private_project.to_owned()),
            class: PathClass::WindowsMounted,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        },
        executables: vec![ExecutableInfo {
            role: "git".to_owned(),
            requested: "git".to_owned(),
            selected: Some(ExecutableCandidate {
                path: windows_git.to_owned(),
                format: ExecutableFormat::Pe,
                origin: ExecutableOrigin::Windows,
            }),
            candidates: vec![
                ExecutableCandidate {
                    path: windows_git.to_owned(),
                    format: ExecutableFormat::Pe,
                    origin: ExecutableOrigin::Windows,
                },
                ExecutableCandidate {
                    path: linux_git.to_owned(),
                    format: ExecutableFormat::Elf,
                    origin: ExecutableOrigin::Linux,
                },
            ],
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        }],
        topology: Topology {
            nodes: vec![TopologyNode {
                id: "private.raw".to_owned(),
                kind: "project".to_owned(),
                label: private_project.to_owned(),
            }],
            edges: Vec::new(),
        },
        evidence: private_evidence(private_project, windows_git),
        findings: vec![Finding {
            id: "PRIVATE".to_owned(),
            title: "Alice private path".to_owned(),
            severity: Severity::Warning,
            summary: format!("Private path: {private_project}"),
            evidence_ids: Vec::new(),
            suggested_actions: vec![format!("Inspect {windows_git}")],
        }],
        probe_failures: vec![ProbeFailure {
            probe: "fixture/v1".to_owned(),
            code: "FIXTURE_FAILED".to_owned(),
            message: format!("Alice failed at {private_project} on WORKSTATION-42"),
        }],
    };
    (report, private_context())
}

fn private_evidence(private_project: &str, windows_git: &str) -> Vec<Evidence> {
    vec![
        Evidence {
            id: "runtime.user".to_owned(),
            probe: "runtime/v2".to_owned(),
            kind: "os-account".to_owned(),
            claim: "current OS account for Alice".to_owned(),
            value: Some("Alice".to_owned()),
            sensitive: true,
        },
        Evidence {
            id: "project.path".to_owned(),
            probe: "path/v1".to_owned(),
            kind: "filesystem".to_owned(),
            claim: "private project path".to_owned(),
            value: Some(private_project.to_owned()),
            sensitive: true,
        },
        Evidence {
            id: "executable.git".to_owned(),
            probe: "executable/v1".to_owned(),
            kind: "executable".to_owned(),
            claim: "git executable".to_owned(),
            value: Some(windows_git.to_owned()),
            sensitive: true,
        },
        Evidence {
            id: "runtime.terminal".to_owned(),
            probe: "runtime/v2".to_owned(),
            kind: "environment".to_owned(),
            claim: "terminal on WORKSTATION-42".to_owned(),
            value: Some("WORKSTATION-42 terminal".to_owned()),
            sensitive: false,
        },
        Evidence {
            id: "fixture.unmodeled-path".to_owned(),
            probe: "fixture/v1".to_owned(),
            kind: "fixture".to_owned(),
            claim: "unexpected absolute value /private/unmodeled/file.txt".to_owned(),
            value: Some("/private/unmodeled/file.txt".to_owned()),
            sensitive: false,
        },
        Evidence {
            id: "fixture.claim-only-path".to_owned(),
            probe: "fixture/v1".to_owned(),
            kind: "fixture".to_owned(),
            claim: "unexpected path=/very/private/location".to_owned(),
            value: None,
            sensitive: false,
        },
    ]
}

fn private_context() -> FixtureRedactionContext {
    FixtureRedactionContext {
        environment: HashMap::from([
            ("USERNAME".to_owned(), "Alice".to_owned()),
            ("USERPROFILE".to_owned(), r"C:\Users\Alice".to_owned()),
            ("HOME".to_owned(), "/home/alice".to_owned()),
            ("COMPUTERNAME".to_owned(), "WORKSTATION-42".to_owned()),
        ]),
    }
}

#[test]
fn redacted_json_contains_no_identity_or_absolute_path() {
    let (report, context) = private_fixture();
    let redacted = redact_with_context(&report, &context);
    let output = json::render(&redacted).expect("redacted report should serialize");

    for private_value in [
        "alice",
        "workstation-42",
        "c:\\users",
        "/home/alice",
        "/mnt/c/users",
        "secret-project",
        "appdata",
        "/usr/bin",
        "/private/unmodeled",
        "/very/private",
    ] {
        assert!(
            !output.to_ascii_lowercase().contains(private_value),
            "shareable JSON leaked {private_value}: {output}"
        );
    }
    assert!(output.contains("[redacted-user]"));
    assert!(output.contains("[redacted-machine]"));
    assert!(output.contains("[windows-mounted-project]"));
    assert!(output.contains("[windows-executable:git:1]"));
    assert!(output.contains("\"product\": \"codex\""));
    assert!(output.contains("\"from\": \"agent.current\""));
}

#[test]
fn shareable_markdown_matches_the_privacy_golden_file() {
    let (report, context) = private_fixture();
    let output = markdown::render_with_context(&report, &context);

    assert_eq!(output, include_str!("fixtures/shareable_report.md"));
}
