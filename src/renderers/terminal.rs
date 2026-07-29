use std::fmt::Write;

use crate::model::{
    AgentEvidenceSource, Confidence, ExecutableFormat, ExecutableInfo, ExecutableOrigin,
    ExecutableResolutionMethod, ObservationStatus, PathClass, Report, RuntimeValueSource, Severity,
};
use crate::renderers::safe::terminal_text;

#[must_use]
pub fn render(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "ExecLocus").expect("writing to String cannot fail");
    writeln!(output, "See what your agent context resolves—and why.\n")
        .expect("writing to String cannot fail");

    render_current_execution(&mut output, report);

    render_agent(&mut output, report);

    writeln!(output, "\nTOOLCHAIN").expect("writing to String cannot fail");
    for executable in &report.executables {
        let selected = executable
            .selected
            .as_ref()
            .map(|candidate| candidate.path.as_str())
            .or(executable.selected_binding.as_deref())
            .unwrap_or("—");
        let kind = executable
            .selected_kind
            .map_or("unknown", |kind| kind.label());
        let note = format!(
            "{} · {kind} · {} candidate(s)",
            executable.selection_state.label(),
            executable.candidates.len()
        );
        line(&mut output, role_label(&executable.role), selected, &note);
        writeln!(
            output,
            "    why       {} · {}",
            terminal_text(&executable.selection_reason),
            terminal_text(&resolution_note(executable))
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "    verify    {}",
            terminal_text(&executable.verification_command)
        )
        .expect("writing to String cannot fail");
        render_candidates(&mut output, executable);
    }

    let finding_count = report.findings.len();
    writeln!(
        output,
        "\n{finding_count} finding{}",
        if finding_count == 1 { "" } else { "s" }
    )
    .expect("writing to String cannot fail");
    for finding in report.findings.iter().take(3) {
        writeln!(
            output,
            "  {:<8} {:<56} {}",
            terminal_text(&finding.id),
            terminal_text(&finding.title),
            severity_label(finding.severity)
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "    impact       {}",
            terminal_text(&finding.summary)
        )
        .expect("writing to String cannot fail");
        for action in &finding.suggested_actions {
            writeln!(output, "    recommended  {}", terminal_text(action))
                .expect("writing to String cannot fail");
        }
        for step in &finding.verification_steps {
            writeln!(output, "    verify       {}", terminal_text(step))
                .expect("writing to String cannot fail");
        }
    }

    if finding_count > 3 {
        writeln!(output, "  … and {} more", finding_count - 3)
            .expect("writing to String cannot fail");
    }
    if !report.probe_failures.is_empty() {
        writeln!(
            output,
            "\n{} optional probe failure(s); partial results shown.",
            report.probe_failures.len()
        )
        .expect("writing to String cannot fail");
    }

    output
}

fn render_current_execution(output: &mut String, report: &Report) {
    writeln!(output, "CURRENT EXECUTION").expect("writing to String cannot fail");
    line(output, "Profile", report.profile.label(), "selected");
    line(
        output,
        "Runtime",
        &runtime_label(report),
        &observation_note(report.runtime.status, report.runtime.kind_source),
    );
    line(
        output,
        "User",
        report.runtime.user.as_deref().unwrap_or("Unknown"),
        report
            .runtime
            .user_source
            .map_or("unavailable", source_label),
    );
    line(
        output,
        "Shell",
        report.runtime.shell.as_deref().unwrap_or("Unknown"),
        report
            .runtime
            .shell_source
            .map_or("unavailable", source_label),
    );
    line(
        output,
        "Terminal",
        report.runtime.terminal.as_deref().unwrap_or("Unknown"),
        if report.runtime.terminal.is_some() {
            "environment hint"
        } else {
            "unavailable"
        },
    );
    line(
        output,
        "Session layer",
        &format!("{:?}", report.runtime.terminal_layer),
        &status_confidence_note(
            report.runtime.terminal_layer_status,
            report.runtime.terminal_layer_confidence,
        ),
    );
    line(
        output,
        "Project",
        report.project.path.as_deref().unwrap_or("Unknown"),
        path_label(report.project.class),
    );
}

fn render_agent(output: &mut String, report: &Report) {
    writeln!(output, "\nAGENT").expect("writing to String cannot fail");
    line(
        output,
        "Product",
        report
            .agent
            .product
            .map_or("Unknown", crate::model::AgentProduct::label),
        &agent_note(
            report.agent.product_status,
            report.agent.product_confidence,
            report.agent.product_source,
        ),
    );
    line(
        output,
        "Runtime",
        &format!("{:?}", report.agent.runtime),
        &status_confidence_note(report.agent.runtime_status, report.agent.runtime_confidence),
    );
    for installation in &report.agent.installations {
        let value = format!("{} candidate(s)", installation.candidates.len());
        line(output, installation.product.label(), &value, "PATH scan");
    }
    for state in &report.agent.state_locations {
        line(output, "Config root", &state.path, path_label(state.class));
    }
}

fn line(output: &mut String, key: &str, value: &str, note: &str) {
    writeln!(
        output,
        "  {:<13} {:<42} {}",
        terminal_text(key),
        terminal_text(value),
        terminal_text(note)
    )
    .expect("writing to String cannot fail");
}

fn render_candidates(output: &mut String, executable: &ExecutableInfo) {
    let selected_path = executable
        .selected
        .as_ref()
        .map(|candidate| candidate.path.as_str());
    let binding_selected = executable.selected_binding.is_some();
    for (index, candidate) in executable.candidates.iter().enumerate() {
        let disposition = if selected_path == Some(candidate.path.as_str()) {
            "selected"
        } else if selected_path.is_some() || binding_selected {
            "losing"
        } else {
            "candidate"
        };
        writeln!(
            output,
            "    {disposition:<9} #{} {}  {} · {} · evidence executable.{}.candidate.{}",
            index + 1,
            terminal_text(&candidate.path),
            origin_label(candidate.origin),
            format_label(candidate.format),
            terminal_text(&executable.role),
            index + 1,
        )
        .expect("writing to String cannot fail");
    }
}

fn resolution_note(executable: &ExecutableInfo) -> String {
    match executable.resolution_method {
        ExecutableResolutionMethod::PathFallback => "generic PATH fallback".to_owned(),
        ExecutableResolutionMethod::ShellContract => {
            let shell = executable
                .resolution_shell
                .as_deref()
                .unwrap_or("unknown shell");
            if executable.shell_session_complete == Some(true) {
                format!("{shell} contract")
            } else {
                format!("{shell} contract · parent session state unavailable")
            }
        }
    }
}

fn runtime_label(report: &Report) -> String {
    report.runtime.distribution.as_ref().map_or_else(
        || format!("{:?}", report.runtime.kind),
        |distribution| format!("{:?} / {distribution}", report.runtime.kind),
    )
}

fn observation_note(status: ObservationStatus, source: Option<RuntimeValueSource>) -> String {
    let status = status_label(status);
    source.map_or_else(
        || status.to_owned(),
        |source| format!("{status} · {}", source_label(source)),
    )
}

fn agent_note(
    status: ObservationStatus,
    confidence: Confidence,
    source: Option<AgentEvidenceSource>,
) -> String {
    let base = status_confidence_note(status, confidence);
    source.map_or(base.clone(), |source| {
        format!("{base} · {}", agent_source_label(source))
    })
}

fn status_confidence_note(status: ObservationStatus, confidence: Confidence) -> String {
    let status = status_label(status);
    if confidence == Confidence::None {
        status.to_owned()
    } else {
        format!("{status} · {} confidence", confidence_label(confidence))
    }
}

const fn status_label(status: ObservationStatus) -> &'static str {
    match status {
        ObservationStatus::Observed => "observed",
        ObservationStatus::Inferred => "inferred",
        ObservationStatus::Unavailable => "unavailable",
        ObservationStatus::Failed => "failed",
    }
}

const fn confidence_label(confidence: Confidence) -> &'static str {
    match confidence {
        Confidence::Certain => "certain",
        Confidence::High => "high",
        Confidence::Medium => "medium",
        Confidence::Low => "low",
        Confidence::None => "none",
    }
}

const fn agent_source_label(source: AgentEvidenceSource) -> &'static str {
    match source {
        AgentEvidenceSource::ProcessAncestry => "process ancestry",
        AgentEvidenceSource::EnvironmentMarker => "environment marker",
    }
}

const fn source_label(source: RuntimeValueSource) -> &'static str {
    match source {
        RuntimeValueSource::TargetPlatform => "target platform",
        RuntimeValueSource::KernelRelease => "kernel release",
        RuntimeValueSource::ProcessAncestry => "process ancestry",
        RuntimeValueSource::OsAccount => "OS account",
        RuntimeValueSource::Environment => "environment hint",
        RuntimeValueSource::OsRelease => "OS release",
    }
}

const fn path_label(class: PathClass) -> &'static str {
    match class {
        PathClass::WindowsNative => "Windows-native",
        PathClass::WindowsMounted => "Windows-mounted",
        PathClass::WslNative => "WSL-native",
        PathClass::WslUnc => "WSL UNC",
        PathClass::LinuxNative => "Linux-native",
        PathClass::Unknown => "unknown",
    }
}

const fn origin_label(origin: ExecutableOrigin) -> &'static str {
    match origin {
        ExecutableOrigin::Windows => "Windows",
        ExecutableOrigin::Linux => "Linux",
        ExecutableOrigin::Script => "Script",
        ExecutableOrigin::Unknown => "unknown",
    }
}

const fn format_label(format: ExecutableFormat) -> &'static str {
    match format {
        ExecutableFormat::Pe => "PE",
        ExecutableFormat::Elf => "ELF",
        ExecutableFormat::Script => "script",
        ExecutableFormat::Unknown => "unknown format",
    }
}

const fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

fn role_label(role: &str) -> &str {
    match role {
        "codex" => "Codex",
        "claude" => "Claude Code",
        "git" => "Git",
        "node" => "Node",
        "npm" => "npm",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        AgentInfo, Confidence, ExecutableCandidate, ExecutableFormat, ExecutableInfo,
        ExecutableOrigin, ExecutableResolutionMethod, ExecutableSelectionKind, Finding,
        ObservationStatus, PathClass, Profile, ProjectInfo, Report, RuntimeInfo, RuntimeKind,
        RuntimeValueSource, Severity, ToolchainState, Topology,
    };

    use super::render;

    #[test]
    fn terminal_output_has_product_and_sections() {
        let report = Report {
            schema_version: "test".to_owned(),
            generated_at_unix_ms: 0,
            profile: Profile::Balanced,
            runtime: RuntimeInfo {
                kind: RuntimeKind::Wsl,
                kind_source: Some(RuntimeValueSource::KernelRelease),
                os_name: "WSL".to_owned(),
                distribution: Some("Ubuntu-Test".to_owned()),
                distribution_source: Some(RuntimeValueSource::Environment),
                user: Some("demo".to_owned()),
                user_source: Some(RuntimeValueSource::OsAccount),
                shell: Some("/bin/bash".to_owned()),
                shell_source: Some(RuntimeValueSource::ProcessAncestry),
                terminal: None,
                terminal_layer: RuntimeKind::Wsl,
                terminal_layer_status: ObservationStatus::Inferred,
                terminal_layer_confidence: Confidence::Certain,
                terminal_layer_source: Some(RuntimeValueSource::ProcessAncestry),
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
            executables: vec![ExecutableInfo {
                role: "node".to_owned(),
                requested: "node".to_owned(),
                selection_state: ToolchainState::Selected,
                selected: Some(ExecutableCandidate {
                    path: "C:\\synthetic\\node\u{1b}[31m.exe\n".to_owned(),
                    format: ExecutableFormat::Pe,
                    origin: ExecutableOrigin::Windows,
                }),
                selected_kind: Some(ExecutableSelectionKind::Application),
                selected_binding: None,
                candidates: vec![
                    ExecutableCandidate {
                        path: "C:\\synthetic\\node\u{1b}[31m.exe\n".to_owned(),
                        format: ExecutableFormat::Pe,
                        origin: ExecutableOrigin::Windows,
                    },
                    ExecutableCandidate {
                        path: "/usr/bin/node\t".to_owned(),
                        format: ExecutableFormat::Elf,
                        origin: ExecutableOrigin::Linux,
                    },
                ],
                resolution_method: ExecutableResolutionMethod::PathFallback,
                resolution_shell: None,
                shell_session_complete: None,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
                selection_reason: "PATH order selected this candidate.".to_owned(),
                verification_command: "type -a -- node".to_owned(),
            }],
            topology: Topology::default(),
            evidence: Vec::new(),
            findings: vec![Finding {
                id: "TOOL001".to_owned(),
                title: "Synthetic tool mismatch".to_owned(),
                severity: Severity::Warning,
                summary: "Direct commands may fail.".to_owned(),
                evidence_ids: Vec::new(),
                suggested_actions: vec!["Inspect the same shell.".to_owned()],
                verification_steps: vec!["Rerun after the change.".to_owned()],
            }],
            probe_failures: Vec::new(),
        };
        let output = render(&report);
        assert!(output.contains("ExecLocus"));
        assert!(output.contains("CURRENT EXECUTION"));
        assert!(output.contains("balanced"));
        assert!(output.contains("observed · kernel release"));
        assert!(output.contains("AGENT"));
        assert!(output.contains("Unknown"));
        assert!(output.contains("TOOLCHAIN"));
        assert!(output.contains("impact       Direct commands may fail."));
        assert!(output.contains("recommended  Inspect the same shell."));
        assert!(output.contains("verify       Rerun after the change."));
        assert!(output.contains("generic PATH fallback"));
        assert!(output.contains("selected"));
        assert!(output.contains("losing"));
        assert!(output.contains("evidence executable.node.candidate.2"));
        assert!(output.contains(r"\u{1b}[31m.exe\n"));
        assert!(output.contains(r"/usr/bin/node\t"));
        assert!(!output.contains('\u{1b}'));
    }
}
