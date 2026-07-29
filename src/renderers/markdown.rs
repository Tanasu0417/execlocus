use std::fmt::Write;

use crate::{
    i18n::{self, Language},
    model::{ExecutableFormat, ExecutableInfo, ExecutableOrigin, Report},
    privacy::{RedactionContext, redact_for_sharing, redact_with_context},
};

#[must_use]
pub fn render(report: &Report) -> String {
    render_with_language(report, Language::English)
}

#[must_use]
pub fn render_with_language(report: &Report, language: Language) -> String {
    let redacted = redact_for_sharing(report);
    let localized = i18n::localize_report(&redacted, language);
    render_redacted(&localized, language)
}

#[must_use]
pub fn render_with_context(report: &Report, context: &dyn RedactionContext) -> String {
    render_redacted(&redact_with_context(report, context), Language::English)
}

#[allow(
    clippy::too_many_lines,
    reason = "the linear shareable-report layout mirrors the rendered section order"
)]
fn render_redacted(report: &Report, language: Language) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# {}\n",
        language.text("ExecLocus shareable report", "ExecLocus 共有用レポート")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "> {}\n",
        language.text(
            "Automatically redacted before rendering. Do not use raw JSON as a public attachment.",
            "描画前に自動匿名化しています。未加工のJSONを公開添付しないでください。"
        )
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "- {}: `{}`",
        language.text("Schema", "形式"),
        cell(&report.schema_version)
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "- {}: `{}`\n",
        language.text("Profile", "利用目的"),
        report.profile.label()
    )
    .expect("writing to String cannot fail");

    render_current_execution(&mut output, report, language);

    render_agent(&mut output, report, language);

    render_toolchain(&mut output, report, language);

    writeln!(output, "\n## {}\n", language.text("Findings", "検出結果"))
        .expect("writing to String cannot fail");
    if report.findings.is_empty() {
        writeln!(
            output,
            "{}",
            language.text("No findings.", "検出結果はありません。")
        )
        .expect("writing to String cannot fail");
    } else {
        for finding in &report.findings {
            writeln!(
                output,
                "- **{}** ({}): {}",
                cell(&finding.id),
                if language == Language::English {
                    format!("{:?}", finding.severity)
                } else {
                    i18n::severity_label(finding.severity, language).to_owned()
                },
                cell(&finding.summary)
            )
            .expect("writing to String cannot fail");
            for action in &finding.suggested_actions {
                writeln!(
                    output,
                    "  - {}: {}",
                    language.text("Recommended", "推奨対応"),
                    cell(action)
                )
                .expect("writing to String cannot fail");
            }
            for step in &finding.verification_steps {
                writeln!(
                    output,
                    "  - {}: {}",
                    language.text("Verify", "再検証"),
                    cell(step)
                )
                .expect("writing to String cannot fail");
            }
        }
    }

    writeln!(
        output,
        "\n## {}\n",
        language.text("Probe status", "調査状態")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "{}",
        if language == Language::English {
            format!(
                "{} optional probe failure(s). Details are omitted from this shareable report.\n",
                report.probe_failures.len()
            )
        } else {
            format!(
                "任意調査の失敗は{}件です。詳細は共有用レポートから省略しています。\n",
                report.probe_failures.len()
            )
        }
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "_{}_",
        language.text(
            "Redacted fields include usernames, home directories, machine names, and absolute paths.",
            "匿名化対象には、ユーザー名、ホームディレクトリ、マシン名、絶対パスが含まれます。"
        )
    )
    .expect("writing to String cannot fail");
    output
}

#[allow(
    clippy::too_many_lines,
    reason = "each toolchain field is rendered together to keep privacy review auditable"
)]
fn render_toolchain(output: &mut String, report: &Report, language: Language) {
    writeln!(
        output,
        "\n## {}\n",
        language.text("Toolchain", "ツールチェーン")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "| {} | {} | {} | {} | {} | {} |",
        language.text("Role", "役割"),
        language.text("State", "状態"),
        language.text("Selected", "選択結果"),
        language.text("Kind", "種類"),
        language.text("Candidates", "候補数"),
        language.text("Why", "理由")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|---|---:|---|").expect("writing to String cannot fail");
    for executable in &report.executables {
        writeln!(
            output,
            "| {} | {} | {} | {} | {} | {} |",
            cell(&executable.role),
            i18n::toolchain_state_label(executable.selection_state, language),
            cell(&selected_value(executable)),
            executable
                .selected_kind
                .map_or("—", |kind| i18n::selection_kind_label(kind, language)),
            executable.candidates.len(),
            cell(&executable.selection_reason),
        )
        .expect("writing to String cannot fail");
    }

    writeln!(
        output,
        "\n### {}\n",
        language.text("Candidate details", "候補詳細")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "| {} | # | {} | {} | {} | {} |",
        language.text("Role", "役割"),
        language.text("Disposition", "判定"),
        language.text("Origin", "由来"),
        language.text("Format", "形式"),
        language.text("Candidate", "候補")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "|---|---:|---|---|---|---|").expect("writing to String cannot fail");
    for executable in &report.executables {
        let selected_path = executable
            .selected
            .as_ref()
            .map(|candidate| candidate.path.as_str());
        let binding_selected = executable.selected_binding.is_some();
        for (index, candidate) in executable.candidates.iter().enumerate() {
            let disposition = if selected_path == Some(candidate.path.as_str()) {
                language.text("selected", "選択済み")
            } else if selected_path.is_some() || binding_selected {
                language.text("not selected", "非選択")
            } else {
                language.text("selection unconfirmed", "選択未確定")
            };
            writeln!(
                output,
                "| {} | {} | {} | {} | {} | {} |",
                cell(&executable.role),
                index + 1,
                disposition,
                origin_label(candidate.origin),
                format_label(candidate.format),
                cell(&candidate.path),
            )
            .expect("writing to String cannot fail");
        }
    }

    writeln!(
        output,
        "\n### {}\n",
        language.text("Independent verification", "独立確認")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "| {} | {} | {} |",
        language.text("Role", "役割"),
        language.text("Command", "コマンド"),
        language.text("Context", "実行場所")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    for executable in &report.executables {
        row(
            output,
            &executable.role,
            &executable.verification_command,
            language.text(
                "run in the same shell session",
                "同じシェルセッションで実行",
            ),
        );
    }
}

fn selected_value(executable: &ExecutableInfo) -> String {
    executable
        .selected
        .as_ref()
        .map(|candidate| candidate.path.clone())
        .or_else(|| executable.selected_binding.clone())
        .unwrap_or_else(|| "—".to_owned())
}

const fn origin_label(origin: ExecutableOrigin) -> &'static str {
    match origin {
        ExecutableOrigin::Windows => "Windows",
        ExecutableOrigin::Linux => "Linux",
        ExecutableOrigin::Script => "Script",
        ExecutableOrigin::Unknown => "Unknown",
    }
}

const fn format_label(format: ExecutableFormat) -> &'static str {
    match format {
        ExecutableFormat::Pe => "PE",
        ExecutableFormat::Elf => "ELF",
        ExecutableFormat::Script => "script",
        ExecutableFormat::Unknown => "unknown",
    }
}

fn render_current_execution(output: &mut String, report: &Report, language: Language) {
    writeln!(
        output,
        "## {}\n",
        language.text("Current execution", "現在の実行環境")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "| {} | {} | {} |",
        language.text("Field", "項目"),
        language.text("Value", "値"),
        language.text("Source", "根拠")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    row(
        output,
        language.text("Runtime", "実行環境"),
        &format!("{:?}", report.runtime.kind),
        &source(report.runtime.kind_source),
    );
    row(
        output,
        language.text("Distribution", "ディストリビューション"),
        report
            .runtime
            .distribution
            .as_deref()
            .unwrap_or(language.text("Unavailable", "利用不可")),
        &source(report.runtime.distribution_source),
    );
    row(
        output,
        language.text("User", "ユーザー"),
        report
            .runtime
            .user
            .as_deref()
            .unwrap_or(language.text("Unavailable", "利用不可")),
        &source(report.runtime.user_source),
    );
    row(
        output,
        language.text("Shell", "シェル"),
        report
            .runtime
            .shell
            .as_deref()
            .unwrap_or(language.text("Unavailable", "利用不可")),
        &source(report.runtime.shell_source),
    );
    row(
        output,
        language.text("Terminal", "ターミナル"),
        report
            .runtime
            .terminal
            .as_deref()
            .unwrap_or(language.text("Unavailable", "利用不可")),
        language.text("environment hint", "環境情報"),
    );
    row(
        output,
        language.text("Session layer", "セッション層"),
        &format!("{:?}", report.runtime.terminal_layer),
        &source(report.runtime.terminal_layer_source),
    );
    row(
        output,
        language.text("Project", "プロジェクト"),
        report
            .project
            .path
            .as_deref()
            .unwrap_or(language.text("Unavailable", "利用不可")),
        &format!("{:?}", report.project.class),
    );
}

fn render_agent(output: &mut String, report: &Report, language: Language) {
    writeln!(
        output,
        "\n## {}\n",
        language.text("Agent execution evidence", "エージェント実行根拠")
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "| {} | {} | {} |",
        language.text("Field", "項目"),
        language.text("Value", "値"),
        language.text("Evidence", "根拠")
    )
    .expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    row(
        output,
        language.text("Product", "製品"),
        report.agent.product.map_or(
            language.text("Unknown", "不明"),
            crate::model::AgentProduct::label,
        ),
        &format!(
            "{:?} / {:?} / {}",
            report.agent.product_status,
            report.agent.product_confidence,
            report
                .agent
                .product_source
                .map_or_else(|| "unavailable".to_owned(), |source| format!("{source:?}"))
        ),
    );
    row(
        output,
        language.text("Runtime", "実行環境"),
        &format!("{:?}", report.agent.runtime),
        &format!(
            "{:?} / {:?}",
            report.agent.runtime_status, report.agent.runtime_confidence
        ),
    );
    for installation in &report.agent.installations {
        row(
            output,
            installation.product.label(),
            &if language == Language::English {
                format!("{} candidate(s)", installation.candidates.len())
            } else {
                format!("{}件の候補", installation.candidates.len())
            },
            &format!("{:?} / {:?}", installation.status, installation.confidence),
        );
    }
    for state in &report.agent.state_locations {
        row(
            output,
            language.text("Primary config root", "主設定ルート"),
            &state.path,
            &format!("{:?} / {:?}", state.class, state.confidence),
        );
    }
}

fn row(output: &mut String, field: &str, value: &str, source: &str) {
    writeln!(
        output,
        "| {} | {} | {} |",
        cell(field),
        cell(value),
        cell(source)
    )
    .expect("writing to String cannot fail");
}

fn cell(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\r' | '\n' | '\t' => escaped.push(' '),
            '\\' => escaped.push_str("\\\\"),
            '|' => escaped.push_str("\\|"),
            '`' => escaped.push_str("\\`"),
            '*' => escaped.push_str("\\*"),
            '_' => escaped.push_str("\\_"),
            '[' => escaped.push_str("\\["),
            ']' => escaped.push_str("\\]"),
            '!' => escaped.push_str("\\!"),
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            character if character.is_control() => {
                write!(escaped, "\\u{{{:x}}}", u32::from(character))
                    .expect("writing to String cannot fail");
            }
            character => escaped.push(character),
        }
    }
    escaped.trim().to_owned()
}

fn source(source: Option<crate::model::RuntimeValueSource>) -> String {
    source.map_or_else(|| "unavailable".to_owned(), |value| format!("{value:?}"))
}

#[cfg(test)]
mod tests {
    use super::cell;

    #[test]
    fn escapes_table_structure_html_links_and_control_characters() {
        let input = "<script>|`code` ![track](https://example.invalid)\u{1b}[31m\nnext";
        let output = cell(input);

        assert_eq!(
            output,
            r"&lt;script&gt;\|\`code\` \!\[track\](https://example.invalid)\u{1b}\[31m next"
        );
        assert!(!output.contains('<'));
        assert!(!output.contains('\u{1b}'));
        assert!(!output.contains("!["));
    }
}
