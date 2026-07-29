use std::fmt::Write;

use crate::{model::Report, renderers::safe::terminal_text, rules::RuleDefinition};

#[must_use]
pub fn render(report: &Report, definition: &RuleDefinition) -> String {
    let mut output = String::new();
    let finding = report
        .findings
        .iter()
        .find(|finding| finding.id == definition.id);

    writeln!(output, "{} — {}", definition.id, definition.title)
        .expect("writing to String cannot fail");
    writeln!(
        output,
        "Status: {}",
        if finding.is_some() {
            "TRIGGERED in the current report"
        } else {
            "not triggered in the current report"
        }
    )
    .expect("writing to String cannot fail");
    writeln!(output, "Category: {}", definition.category).expect("writing to String cannot fail");
    writeln!(output, "Default severity: {}", definition.default_severity)
        .expect("writing to String cannot fail");

    writeln!(output, "\nWHY THIS RULE EXISTS").expect("writing to String cannot fail");
    writeln!(output, "  {}", definition.rationale).expect("writing to String cannot fail");

    writeln!(output, "\nREQUIRED EVIDENCE").expect("writing to String cannot fail");
    for required in definition.required_evidence {
        writeln!(output, "  - {required}").expect("writing to String cannot fail");
    }

    if let Some(finding) = finding {
        writeln!(output, "\nCURRENT FINDING").expect("writing to String cannot fail");
        writeln!(output, "  Severity: {:?}", finding.severity)
            .expect("writing to String cannot fail");
        writeln!(output, "  Summary: {}", terminal_text(&finding.summary))
            .expect("writing to String cannot fail");

        writeln!(output, "\nOBSERVED EVIDENCE").expect("writing to String cannot fail");
        for evidence_id in &finding.evidence_ids {
            if let Some(evidence) = report.evidence.iter().find(|item| item.id == *evidence_id) {
                let value = evidence
                    .value
                    .as_deref()
                    .map_or("<value unavailable>".to_owned(), terminal_text);
                writeln!(
                    output,
                    "  - {}: {} = {} [{}]",
                    evidence.id,
                    terminal_text(&evidence.claim),
                    value,
                    evidence.probe
                )
                .expect("writing to String cannot fail");
            } else {
                writeln!(output, "  - {evidence_id}: <evidence unavailable>")
                    .expect("writing to String cannot fail");
            }
        }
    } else {
        writeln!(output, "\nCURRENT REPORT NOTE").expect("writing to String cannot fail");
        writeln!(
            output,
            "  No finding was emitted. This can mean the condition is absent or that required evidence is unavailable; it is not a pass/fail certification."
        )
        .expect("writing to String cannot fail");
    }

    writeln!(output, "\nREAD-ONLY SUGGESTED ACTIONS").expect("writing to String cannot fail");
    if let Some(finding) = finding {
        for action in &finding.suggested_actions {
            writeln!(output, "  - {}", terminal_text(action))
                .expect("writing to String cannot fail");
        }
    } else {
        for action in definition.suggested_actions {
            writeln!(output, "  - {}", terminal_text(action))
                .expect("writing to String cannot fail");
        }
    }
    writeln!(
        output,
        "\nExecLocus does not modify PATH, configuration, files, or installations."
    )
    .expect("writing to String cannot fail");

    output
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            AgentInfo, Confidence, Evidence, Finding, ObservationStatus, PathClass, Profile,
            ProjectInfo, Report, RuntimeInfo, RuntimeKind, Severity, Topology,
        },
        rules::definition,
    };

    use super::render;

    fn report(findings: Vec<Finding>) -> Report {
        Report {
            schema_version: "test".to_owned(),
            generated_at_unix_ms: 0,
            profile: Profile::Balanced,
            runtime: RuntimeInfo {
                kind: RuntimeKind::Wsl,
                kind_source: None,
                os_name: "WSL".to_owned(),
                distribution: Some("Synthetic Linux".to_owned()),
                distribution_source: None,
                user: Some("demo".to_owned()),
                user_source: None,
                shell: Some("bash".to_owned()),
                shell_source: None,
                terminal: None,
                terminal_layer: RuntimeKind::Wsl,
                terminal_layer_status: ObservationStatus::Observed,
                terminal_layer_confidence: Confidence::Certain,
                terminal_layer_source: None,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            agent: AgentInfo::default(),
            project: ProjectInfo {
                path: Some("/mnt/c/demo/project".to_owned()),
                class: PathClass::WindowsMounted,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            executables: Vec::new(),
            topology: Topology::default(),
            evidence: vec![Evidence {
                id: "project.path".to_owned(),
                probe: "path/test".to_owned(),
                kind: "filesystem".to_owned(),
                claim: "project path\u{1b}[31m classified".to_owned(),
                value: Some("/mnt/c/demo/project\nsecond-line".to_owned()),
                sensitive: true,
            }],
            findings,
            probe_failures: Vec::new(),
        }
    }

    #[test]
    fn triggered_rule_includes_only_referenced_observed_evidence_and_actions() {
        let finding = Finding {
            id: "FS001".to_owned(),
            title: "Synthetic finding".to_owned(),
            severity: Severity::Info,
            summary: "Synthetic summary".to_owned(),
            evidence_ids: vec!["project.path".to_owned()],
            suggested_actions: vec!["Keep the intentional shared path.".to_owned()],
        };

        let output = render(&report(vec![finding]), definition("FS001").unwrap());

        assert!(output.contains("TRIGGERED in the current report"));
        assert!(output.contains("project.path"));
        assert!(output.contains("Keep the intentional shared path."));
        assert!(output.contains(r"\u{1b}[31m"));
        assert!(output.contains(r"project\nsecond-line"));
        assert!(!output.contains('\u{1b}'));
    }

    #[test]
    fn non_triggered_rule_explains_that_missing_evidence_is_not_a_pass() {
        let output = render(&report(Vec::new()), definition("ENV002").unwrap());

        assert!(output.contains("not triggered in the current report"));
        assert!(output.contains("required evidence is unavailable"));
        assert!(output.contains("READ-ONLY SUGGESTED ACTIONS"));
    }
}
