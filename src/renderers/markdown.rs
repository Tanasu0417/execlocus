use std::fmt::Write;

use crate::{
    model::{ExecutableOrigin, Report},
    privacy::{RedactionContext, redact_for_sharing, redact_with_context},
};

#[must_use]
pub fn render(report: &Report) -> String {
    render_redacted(&redact_for_sharing(report))
}

#[must_use]
pub fn render_with_context(report: &Report, context: &dyn RedactionContext) -> String {
    render_redacted(&redact_with_context(report, context))
}

fn render_redacted(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "# ExecLocus shareable report\n").expect("writing to String cannot fail");
    writeln!(
        output,
        "> Automatically redacted before rendering. Do not use raw JSON as a public attachment.\n"
    )
    .expect("writing to String cannot fail");
    writeln!(output, "- Schema: `{}`", cell(&report.schema_version))
        .expect("writing to String cannot fail");
    writeln!(output, "- Profile: `{}`\n", report.profile.label())
        .expect("writing to String cannot fail");

    render_current_execution(&mut output, report);

    render_agent(&mut output, report);

    writeln!(output, "\n## Toolchain\n").expect("writing to String cannot fail");
    writeln!(output, "| Role | Selected | Origin |").expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    for executable in &report.executables {
        let (selected, origin) = executable
            .selected
            .as_ref()
            .map_or(("Unavailable", ExecutableOrigin::Unknown), |candidate| {
                (candidate.path.as_str(), candidate.origin)
            });
        row(
            &mut output,
            &executable.role,
            selected,
            &format!("{origin:?}"),
        );
    }

    writeln!(output, "\n## Findings\n").expect("writing to String cannot fail");
    if report.findings.is_empty() {
        writeln!(output, "No findings.").expect("writing to String cannot fail");
    } else {
        for finding in &report.findings {
            writeln!(
                output,
                "- **{}** ({:?}): {}",
                cell(&finding.id),
                finding.severity,
                cell(&finding.summary)
            )
            .expect("writing to String cannot fail");
        }
    }

    writeln!(output, "\n## Probe status\n").expect("writing to String cannot fail");
    writeln!(
        output,
        "{} optional probe failure(s). Details are omitted from this shareable report.\n",
        report.probe_failures.len()
    )
    .expect("writing to String cannot fail");
    writeln!(
        output,
        "_Redacted fields include usernames, home directories, machine names, and absolute paths._"
    )
    .expect("writing to String cannot fail");
    output
}

fn render_current_execution(output: &mut String, report: &Report) {
    writeln!(output, "## Current execution\n").expect("writing to String cannot fail");
    writeln!(output, "| Field | Value | Source |").expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    row(
        output,
        "Runtime",
        &format!("{:?}", report.runtime.kind),
        &source(report.runtime.kind_source),
    );
    row(
        output,
        "Distribution",
        report
            .runtime
            .distribution
            .as_deref()
            .unwrap_or("Unavailable"),
        &source(report.runtime.distribution_source),
    );
    row(
        output,
        "User",
        report.runtime.user.as_deref().unwrap_or("Unavailable"),
        &source(report.runtime.user_source),
    );
    row(
        output,
        "Shell",
        report.runtime.shell.as_deref().unwrap_or("Unavailable"),
        &source(report.runtime.shell_source),
    );
    row(
        output,
        "Terminal",
        report.runtime.terminal.as_deref().unwrap_or("Unavailable"),
        "environment hint",
    );
    row(
        output,
        "Session layer",
        &format!("{:?}", report.runtime.terminal_layer),
        &source(report.runtime.terminal_layer_source),
    );
    row(
        output,
        "Project",
        report.project.path.as_deref().unwrap_or("Unavailable"),
        &format!("{:?}", report.project.class),
    );
}

fn render_agent(output: &mut String, report: &Report) {
    writeln!(output, "\n## Agent execution evidence\n").expect("writing to String cannot fail");
    writeln!(output, "| Field | Value | Evidence |").expect("writing to String cannot fail");
    writeln!(output, "|---|---|---|").expect("writing to String cannot fail");
    row(
        output,
        "Product",
        report
            .agent
            .product
            .map_or("Unknown", crate::model::AgentProduct::label),
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
        "Runtime",
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
            &format!("{} candidate(s)", installation.candidates.len()),
            &format!("{:?} / {:?}", installation.status, installation.confidence),
        );
    }
    for state in &report.agent.state_locations {
        row(
            output,
            "Primary config root",
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
