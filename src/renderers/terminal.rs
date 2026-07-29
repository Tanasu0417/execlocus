use std::fmt::Write;

use crate::i18n::{self, Language};
use crate::model::{
    AgentEvidenceSource, Confidence, ExecutableFormat, ExecutableInfo, ExecutableOrigin,
    ExecutableResolutionMethod, ObservationStatus, PathClass, Report, RuntimeValueSource,
};
use crate::renderers::safe::terminal_text;

#[must_use]
pub fn render(report: &Report) -> String {
    render_with_language(report, Language::English)
}

#[must_use]
#[allow(
    clippy::too_many_lines,
    reason = "the terminal hierarchy is intentionally linear and follows display order"
)]
pub fn render_with_language(report: &Report, language: Language) -> String {
    let report = i18n::localize_report(report, language);
    let mut output = String::new();
    writeln!(output, "ExecLocus").expect("writing to String cannot fail");
    writeln!(
        output,
        "{}\n",
        language.text(
            "See what your agent context resolves—and why.",
            "エージェントの実行環境が何を選び、その理由が何かを確認します。"
        )
    )
    .expect("writing to String cannot fail");

    render_current_execution(&mut output, &report, language);

    render_agent(&mut output, &report, language);

    writeln!(output, "\n{}", language.text("TOOLCHAIN", "ツールチェーン"))
        .expect("writing to String cannot fail");
    for executable in &report.executables {
        let selected = executable
            .selected
            .as_ref()
            .map(|candidate| candidate.path.as_str())
            .or(executable.selected_binding.as_deref())
            .unwrap_or("—");
        let kind = executable
            .selected_kind
            .map_or(language.text("unknown", "不明"), |kind| {
                i18n::selection_kind_label(kind, language)
            });
        let note = format!(
            "{} · {kind} · {}",
            i18n::toolchain_state_label(executable.selection_state, language),
            if language == Language::English {
                format!("{} candidate(s)", executable.candidates.len())
            } else {
                format!("{}件の候補", executable.candidates.len())
            }
        );
        line(&mut output, role_label(&executable.role), selected, &note);
        writeln!(
            output,
            "    {:<10} {} · {}",
            language.text("why", "理由"),
            terminal_text(&executable.selection_reason),
            terminal_text(&resolution_note(executable, language))
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "    {:<10} {}",
            language.text("verify", "確認"),
            terminal_text(&executable.verification_command)
        )
        .expect("writing to String cannot fail");
        render_candidates(&mut output, executable, language);
    }

    let finding_count = report.findings.len();
    writeln!(
        output,
        "\n{}",
        if language == Language::English {
            format!(
                "{finding_count} finding{}",
                if finding_count == 1 { "" } else { "s" }
            )
        } else {
            format!("検出結果 {finding_count}件")
        }
    )
    .expect("writing to String cannot fail");
    for finding in report.findings.iter().take(3) {
        writeln!(
            output,
            "  {:<8} {:<56} {}",
            terminal_text(&finding.id),
            terminal_text(&finding.title),
            i18n::severity_label(finding.severity, language)
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "    {:<12} {}",
            language.text("impact", "影響"),
            terminal_text(&finding.summary)
        )
        .expect("writing to String cannot fail");
        for action in &finding.suggested_actions {
            writeln!(
                output,
                "    {:<12} {}",
                language.text("recommended", "推奨対応"),
                terminal_text(action)
            )
            .expect("writing to String cannot fail");
        }
        for step in &finding.verification_steps {
            writeln!(
                output,
                "    {:<12} {}",
                language.text("verify", "再検証"),
                terminal_text(step)
            )
            .expect("writing to String cannot fail");
        }
    }

    if finding_count > 3 {
        writeln!(
            output,
            "  {}",
            if language == Language::English {
                format!("… and {} more", finding_count - 3)
            } else {
                format!("… ほか{}件", finding_count - 3)
            }
        )
        .expect("writing to String cannot fail");
    }
    if !report.probe_failures.is_empty() {
        writeln!(
            output,
            "\n{}",
            if language == Language::English {
                format!(
                    "{} optional probe failure(s); partial results shown.",
                    report.probe_failures.len()
                )
            } else {
                format!(
                    "任意調査の失敗が{}件あります。取得できた結果を表示します。",
                    report.probe_failures.len()
                )
            }
        )
        .expect("writing to String cannot fail");
    }

    output
}

fn render_current_execution(output: &mut String, report: &Report, language: Language) {
    writeln!(
        output,
        "{}",
        language.text("CURRENT EXECUTION", "現在の実行環境")
    )
    .expect("writing to String cannot fail");
    line(
        output,
        language.text("Profile", "利用目的"),
        report.profile.label(),
        language.text("selected", "選択済み"),
    );
    line(
        output,
        language.text("Runtime", "実行環境"),
        &runtime_label(report, language),
        &observation_note(report.runtime.status, report.runtime.kind_source, language),
    );
    line(
        output,
        language.text("User", "ユーザー"),
        report
            .runtime
            .user
            .as_deref()
            .unwrap_or(language.text("Unknown", "不明")),
        report
            .runtime
            .user_source
            .map_or(language.text("unavailable", "利用不可"), |source| {
                source_label(source, language)
            }),
    );
    line(
        output,
        language.text("Shell", "シェル"),
        report
            .runtime
            .shell
            .as_deref()
            .unwrap_or(language.text("Unknown", "不明")),
        report
            .runtime
            .shell_source
            .map_or(language.text("unavailable", "利用不可"), |source| {
                source_label(source, language)
            }),
    );
    line(
        output,
        language.text("Terminal", "ターミナル"),
        report
            .runtime
            .terminal
            .as_deref()
            .unwrap_or(language.text("Unknown", "不明")),
        if report.runtime.terminal.is_some() {
            language.text("environment hint", "環境情報")
        } else {
            language.text("unavailable", "利用不可")
        },
    );
    line(
        output,
        language.text("Session layer", "セッション層"),
        runtime_kind_label(report.runtime.terminal_layer, language),
        &status_confidence_note(
            report.runtime.terminal_layer_status,
            report.runtime.terminal_layer_confidence,
            language,
        ),
    );
    line(
        output,
        language.text("Project", "プロジェクト"),
        report
            .project
            .path
            .as_deref()
            .unwrap_or(language.text("Unknown", "不明")),
        path_label(report.project.class, language),
    );
}

fn render_agent(output: &mut String, report: &Report, language: Language) {
    writeln!(output, "\n{}", language.text("AGENT", "エージェント"))
        .expect("writing to String cannot fail");
    line(
        output,
        language.text("Product", "製品"),
        report.agent.product.map_or(
            language.text("Unknown", "不明"),
            crate::model::AgentProduct::label,
        ),
        &agent_note(
            report.agent.product_status,
            report.agent.product_confidence,
            report.agent.product_source,
            language,
        ),
    );
    line(
        output,
        language.text("Runtime", "実行環境"),
        runtime_kind_label(report.agent.runtime, language),
        &status_confidence_note(
            report.agent.runtime_status,
            report.agent.runtime_confidence,
            language,
        ),
    );
    for installation in &report.agent.installations {
        let value = if language == Language::English {
            format!("{} candidate(s)", installation.candidates.len())
        } else {
            format!("{}件の候補", installation.candidates.len())
        };
        line(
            output,
            installation.product.label(),
            &value,
            language.text("PATH scan", "PATH調査"),
        );
    }
    for state in &report.agent.state_locations {
        line(
            output,
            language.text("Config root", "設定ルート"),
            &state.path,
            path_label(state.class, language),
        );
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

fn render_candidates(output: &mut String, executable: &ExecutableInfo, language: Language) {
    let selected_path = executable
        .selected
        .as_ref()
        .map(|candidate| candidate.path.as_str());
    let binding_selected = executable.selected_binding.is_some();
    for (index, candidate) in executable.candidates.iter().enumerate() {
        let disposition = if selected_path == Some(candidate.path.as_str()) {
            language.text("selected", "選択済み")
        } else if selected_path.is_some() || binding_selected {
            language.text("losing", "非選択")
        } else {
            language.text("candidate", "候補")
        };
        writeln!(
            output,
            "    {disposition:<9} #{} {}  {} · {} · {} executable.{}.candidate.{}",
            index + 1,
            terminal_text(&candidate.path),
            origin_label(candidate.origin, language),
            format_label(candidate.format, language),
            language.text("evidence", "根拠"),
            terminal_text(&executable.role),
            index + 1,
        )
        .expect("writing to String cannot fail");
    }
}

fn resolution_note(executable: &ExecutableInfo, language: Language) -> String {
    match executable.resolution_method {
        ExecutableResolutionMethod::PathFallback => language
            .text("generic PATH fallback", "一般的なPATH予備判定")
            .to_owned(),
        ExecutableResolutionMethod::ShellContract => {
            let shell = executable
                .resolution_shell
                .as_deref()
                .unwrap_or("unknown shell");
            if executable.shell_session_complete == Some(true) {
                format!("{shell} {}", language.text("contract", "規則"))
            } else {
                format!(
                    "{shell} {} · {}",
                    language.text("contract", "規則"),
                    language.text("parent session state unavailable", "親セッション情報なし")
                )
            }
        }
    }
}

fn runtime_label(report: &Report, language: Language) -> String {
    report.runtime.distribution.as_ref().map_or_else(
        || runtime_kind_label(report.runtime.kind, language).to_owned(),
        |distribution| {
            format!(
                "{} / {distribution}",
                runtime_kind_label(report.runtime.kind, language)
            )
        },
    )
}

fn observation_note(
    status: ObservationStatus,
    source: Option<RuntimeValueSource>,
    language: Language,
) -> String {
    let status = status_label(status, language);
    source.map_or_else(
        || status.to_owned(),
        |source| format!("{status} · {}", source_label(source, language)),
    )
}

fn agent_note(
    status: ObservationStatus,
    confidence: Confidence,
    source: Option<AgentEvidenceSource>,
    language: Language,
) -> String {
    let base = status_confidence_note(status, confidence, language);
    source.map_or(base.clone(), |source| {
        format!("{base} · {}", agent_source_label(source, language))
    })
}

fn status_confidence_note(
    status: ObservationStatus,
    confidence: Confidence,
    language: Language,
) -> String {
    let status = status_label(status, language);
    if confidence == Confidence::None {
        status.to_owned()
    } else {
        format!(
            "{status} · {}{}",
            confidence_label(confidence, language),
            language.text(" confidence", "の確度")
        )
    }
}

const fn status_label(status: ObservationStatus, language: Language) -> &'static str {
    match (status, language) {
        (ObservationStatus::Observed, Language::English) => "observed",
        (ObservationStatus::Observed, Language::Japanese) => "観測済み",
        (ObservationStatus::Inferred, Language::English) => "inferred",
        (ObservationStatus::Inferred, Language::Japanese) => "推定",
        (ObservationStatus::Unavailable, Language::English) => "unavailable",
        (ObservationStatus::Unavailable, Language::Japanese) => "利用不可",
        (ObservationStatus::Failed, Language::English) => "failed",
        (ObservationStatus::Failed, Language::Japanese) => "失敗",
    }
}

const fn confidence_label(confidence: Confidence, language: Language) -> &'static str {
    match (confidence, language) {
        (Confidence::Certain, Language::English) => "certain",
        (Confidence::Certain, Language::Japanese) => "確実",
        (Confidence::High, Language::English) => "high",
        (Confidence::High, Language::Japanese) => "高",
        (Confidence::Medium, Language::English) => "medium",
        (Confidence::Medium, Language::Japanese) => "中",
        (Confidence::Low, Language::English) => "low",
        (Confidence::Low, Language::Japanese) => "低",
        (Confidence::None, Language::English) => "none",
        (Confidence::None, Language::Japanese) => "なし",
    }
}

const fn agent_source_label(source: AgentEvidenceSource, language: Language) -> &'static str {
    match (source, language) {
        (AgentEvidenceSource::ProcessAncestry, Language::English) => "process ancestry",
        (AgentEvidenceSource::ProcessAncestry, Language::Japanese) => "親プロセス関係",
        (AgentEvidenceSource::EnvironmentMarker, Language::English) => "environment marker",
        (AgentEvidenceSource::EnvironmentMarker, Language::Japanese) => "環境マーカー",
    }
}

const fn source_label(source: RuntimeValueSource, language: Language) -> &'static str {
    match (source, language) {
        (RuntimeValueSource::TargetPlatform, Language::English) => "target platform",
        (RuntimeValueSource::TargetPlatform, Language::Japanese) => "対象プラットフォーム",
        (RuntimeValueSource::KernelRelease, Language::English) => "kernel release",
        (RuntimeValueSource::KernelRelease, Language::Japanese) => "カーネル情報",
        (RuntimeValueSource::ProcessAncestry, Language::English) => "process ancestry",
        (RuntimeValueSource::ProcessAncestry, Language::Japanese) => "親プロセス関係",
        (RuntimeValueSource::OsAccount, Language::English) => "OS account",
        (RuntimeValueSource::OsAccount, Language::Japanese) => "OSアカウント",
        (RuntimeValueSource::Environment, Language::English) => "environment hint",
        (RuntimeValueSource::Environment, Language::Japanese) => "環境情報",
        (RuntimeValueSource::OsRelease, Language::English) => "OS release",
        (RuntimeValueSource::OsRelease, Language::Japanese) => "OSリリース情報",
    }
}

const fn path_label(class: PathClass, language: Language) -> &'static str {
    match (class, language) {
        (PathClass::WindowsNative, Language::English) => "Windows-native",
        (PathClass::WindowsNative, Language::Japanese) => "Windowsネイティブ",
        (PathClass::WindowsMounted, Language::English) => "Windows-mounted",
        (PathClass::WindowsMounted, Language::Japanese) => "Windowsマウント",
        (PathClass::WslNative, Language::English) => "WSL-native",
        (PathClass::WslNative, Language::Japanese) => "WSLネイティブ",
        (PathClass::WslUnc, _) => "WSL UNC",
        (PathClass::LinuxNative, Language::English) => "Linux-native",
        (PathClass::LinuxNative, Language::Japanese) => "Linuxネイティブ",
        (PathClass::Unknown, Language::English) => "unknown",
        (PathClass::Unknown, Language::Japanese) => "不明",
    }
}

const fn origin_label(origin: ExecutableOrigin, language: Language) -> &'static str {
    match (origin, language) {
        (ExecutableOrigin::Windows, _) => "Windows",
        (ExecutableOrigin::Linux, _) => "Linux",
        (ExecutableOrigin::Script, Language::English) => "Script",
        (ExecutableOrigin::Script, Language::Japanese) => "スクリプト",
        (ExecutableOrigin::Unknown, Language::English) => "unknown",
        (ExecutableOrigin::Unknown, Language::Japanese) => "不明",
    }
}

const fn format_label(format: ExecutableFormat, language: Language) -> &'static str {
    match (format, language) {
        (ExecutableFormat::Pe, _) => "PE",
        (ExecutableFormat::Elf, _) => "ELF",
        (ExecutableFormat::Script, Language::English) => "script",
        (ExecutableFormat::Script, Language::Japanese) => "スクリプト",
        (ExecutableFormat::Unknown, Language::English) => "unknown format",
        (ExecutableFormat::Unknown, Language::Japanese) => "形式不明",
    }
}

const fn runtime_kind_label(
    runtime: crate::model::RuntimeKind,
    language: Language,
) -> &'static str {
    match (runtime, language) {
        (crate::model::RuntimeKind::WindowsNative, Language::English) => "WindowsNative",
        (crate::model::RuntimeKind::WindowsNative, Language::Japanese) => "Windowsネイティブ",
        (crate::model::RuntimeKind::Wsl, _) => "WSL",
        (crate::model::RuntimeKind::LinuxNative, Language::English) => "LinuxNative",
        (crate::model::RuntimeKind::LinuxNative, Language::Japanese) => "Linuxネイティブ",
        (crate::model::RuntimeKind::Unknown, Language::English) => "Unknown",
        (crate::model::RuntimeKind::Unknown, Language::Japanese) => "不明",
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
