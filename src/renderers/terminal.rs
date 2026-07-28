use std::fmt::Write;

use crate::model::{ExecutableOrigin, PathClass, Report, RuntimeValueSource, Severity};

#[must_use]
pub fn render(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "ExecLocus").expect("writing to String cannot fail");
    writeln!(output, "See what your agent context resolves—and why.\n")
        .expect("writing to String cannot fail");

    writeln!(output, "CURRENT EXECUTION").expect("writing to String cannot fail");
    line(&mut output, "Runtime", &runtime_label(report), "observed");
    line(
        &mut output,
        "User",
        report.runtime.user.as_deref().unwrap_or("Unknown"),
        report
            .runtime
            .user_source
            .map_or("unavailable", source_label),
    );
    line(
        &mut output,
        "Shell",
        report.runtime.shell.as_deref().unwrap_or("Unknown"),
        report
            .runtime
            .shell_source
            .map_or("unavailable", source_label),
    );
    line(
        &mut output,
        "Terminal",
        report.runtime.terminal.as_deref().unwrap_or("Unknown"),
        if report.runtime.terminal.is_some() {
            "environment hint"
        } else {
            "unavailable"
        },
    );
    line(
        &mut output,
        "Project",
        report.project.path.as_deref().unwrap_or("Unknown"),
        path_label(report.project.class),
    );

    writeln!(output, "\nTOOLCHAIN").expect("writing to String cannot fail");
    for executable in &report.executables {
        let (path, origin) = executable
            .selected
            .as_ref()
            .map_or(("Not found", ExecutableOrigin::Unknown), |selected| {
                (selected.path.as_str(), selected.origin)
            });
        line(
            &mut output,
            &capitalize(&executable.role),
            path,
            origin_label(origin),
        );
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
            finding.id,
            finding.title,
            severity_label(finding.severity)
        )
        .expect("writing to String cannot fail");
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

fn line(output: &mut String, key: &str, value: &str, note: &str) {
    writeln!(output, "  {key:<13} {value:<42} {note}").expect("writing to String cannot fail");
}

fn runtime_label(report: &Report) -> String {
    report.runtime.distribution.as_ref().map_or_else(
        || format!("{:?}", report.runtime.kind),
        |distribution| format!("{:?} / {distribution}", report.runtime.kind),
    )
}

const fn source_label(source: RuntimeValueSource) -> &'static str {
    match source {
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

const fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

fn capitalize(value: &str) -> String {
    let mut characters = value.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + characters.as_str()
    })
}

#[cfg(test)]
mod tests {
    use crate::model::{
        Confidence, ObservationStatus, PathClass, Profile, ProjectInfo, Report, RuntimeInfo,
        RuntimeKind, RuntimeValueSource, Topology,
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
                os_name: "WSL".to_owned(),
                distribution: Some("Ubuntu-Test".to_owned()),
                distribution_source: Some(RuntimeValueSource::Environment),
                user: Some("demo".to_owned()),
                user_source: Some(RuntimeValueSource::OsAccount),
                shell: Some("/bin/bash".to_owned()),
                shell_source: Some(RuntimeValueSource::ProcessAncestry),
                terminal: None,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            project: ProjectInfo {
                path: Some("/mnt/c/demo/project".to_owned()),
                class: PathClass::WindowsMounted,
                status: ObservationStatus::Observed,
                confidence: Confidence::Certain,
            },
            executables: Vec::new(),
            topology: Topology::default(),
            evidence: Vec::new(),
            findings: Vec::new(),
            probe_failures: Vec::new(),
        };
        let output = render(&report);
        assert!(output.contains("ExecLocus"));
        assert!(output.contains("CURRENT EXECUTION"));
        assert!(output.contains("TOOLCHAIN"));
    }
}
