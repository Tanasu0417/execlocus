use std::fmt::Write;

use crate::{
    i18n::{self, Language},
    model::Report,
    renderers::safe::terminal_text,
    rules::RuleDefinition,
};

#[must_use]
pub fn render(report: &Report, definition: &RuleDefinition) -> String {
    render_with_language(report, definition, Language::English)
}

#[must_use]
#[allow(
    clippy::too_many_lines,
    reason = "the linear explanation layout mirrors the user-visible section order"
)]
pub fn render_with_language(
    report: &Report,
    definition: &RuleDefinition,
    language: Language,
) -> String {
    let report = i18n::localize_report(report, language);
    let mut output = String::new();
    let finding = report
        .findings
        .iter()
        .find(|finding| finding.id == definition.id);

    writeln!(
        output,
        "{} — {}",
        definition.id,
        i18n::rule_title(definition, language)
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "{}: {}",
        language.text("Status", "状態"),
        if finding.is_some() {
            language.text("TRIGGERED in the current report", "現在のレポートで検出")
        } else {
            language.text(
                "not triggered in the current report",
                "現在のレポートでは未検出",
            )
        }
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "{}: {}",
        language.text("Category", "分類"),
        definition.category
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "{}: {}",
        language.text("Default severity", "標準重要度"),
        definition.default_severity
    )
    .expect("writing to String cannot fail");

    writeln!(
        output,
        "\n{}",
        language.text("WHY THIS RULE EXISTS", "このルールが必要な理由")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "  {}", i18n::rule_rationale(definition, language))
        .expect("writing to String cannot fail");

    writeln!(
        output,
        "\n{}",
        language.text("REQUIRED EVIDENCE", "必要な根拠")
    )
    .expect("writing to String cannot fail");
    for required in i18n::rule_required_evidence(definition, language) {
        writeln!(output, "  - {required}").expect("writing to String cannot fail");
    }

    if let Some(finding) = finding {
        writeln!(
            output,
            "\n{}",
            language.text("CURRENT FINDING", "現在の検出結果")
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  {}: {}",
            language.text("Severity", "重要度"),
            i18n::severity_label(finding.severity, language)
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  {}: {}",
            language.text("Summary", "概要"),
            terminal_text(&finding.summary)
        )
        .expect("writing to String cannot fail");

        writeln!(
            output,
            "\n{}",
            language.text("OBSERVED EVIDENCE", "観測された根拠")
        )
        .expect("writing to String cannot fail");
        for evidence_id in &finding.evidence_ids {
            if let Some(evidence) = report.evidence.iter().find(|item| item.id == *evidence_id) {
                let value = evidence.value.as_deref().map_or(
                    language
                        .text("<value unavailable>", "<値を利用できません>")
                        .to_owned(),
                    terminal_text,
                );
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
                writeln!(
                    output,
                    "  - {evidence_id}: {}",
                    language.text("<evidence unavailable>", "<根拠を利用できません>")
                )
                .expect("writing to String cannot fail");
            }
        }
    } else {
        writeln!(
            output,
            "\n{}",
            language.text("CURRENT REPORT NOTE", "現在のレポートに関する注意")
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  {}",
            language.text(
                "No finding was emitted. This can mean the condition is absent or that required evidence is unavailable; it is not a pass/fail certification.",
                "検出結果はありません。条件に該当しない場合と、必要な根拠が不足する場合の両方があるため、合否認証ではありません。"
            )
        )
        .expect("writing to String cannot fail");
    }

    if let Some(finding) = finding {
        writeln!(output, "\n{}", language.text("REVERIFICATION", "再検証"))
            .expect("writing to String cannot fail");
        for step in &finding.verification_steps {
            writeln!(output, "  - {}", terminal_text(step)).expect("writing to String cannot fail");
        }
    }

    writeln!(
        output,
        "\n{}",
        language.text("READ-ONLY SUGGESTED ACTIONS", "読み取り専用の推奨対応")
    )
    .expect("writing to String cannot fail");
    if let Some(finding) = finding {
        for action in &finding.suggested_actions {
            writeln!(output, "  - {}", terminal_text(action))
                .expect("writing to String cannot fail");
        }
    } else {
        for action in i18n::rule_suggested_actions(definition, language) {
            writeln!(output, "  - {}", terminal_text(action))
                .expect("writing to String cannot fail");
        }
    }
    writeln!(
        output,
        "\n{}",
        language.text(
            "ExecLocus does not modify PATH, configuration, files, or installations.",
            "ExecLocusはPATH、設定、ファイル、インストール内容を変更しません。"
        )
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
            verification_steps: vec!["Rerun ExecLocus after the change.".to_owned()],
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
