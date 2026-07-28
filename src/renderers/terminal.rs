use std::fmt::Write;

use crate::model::{ExecutableOrigin, PathClass, Report, Severity};

#[must_use]
pub fn render(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "ExecLocus").expect("writing to String cannot fail");
    writeln!(output, "See where your agent actually executes.\n")
        .expect("writing to String cannot fail");

    writeln!(output, "CURRENT EXECUTION").expect("writing to String cannot fail");
    line(&mut output, "Runtime", &runtime_label(report), "observed");
    line(
        &mut output,
        "User",
        report.runtime.user.as_deref().unwrap_or("Unknown"),
        status_label(report.runtime.user.is_some()),
    );
    line(
        &mut output,
        "Shell",
        report.runtime.shell.as_deref().unwrap_or("Unknown"),
        if report.runtime.shell.is_some() {
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

const fn status_label(available: bool) -> &'static str {
    if available { "observed" } else { "unavailable" }
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
    use crate::{collect_report, model::Profile};

    use super::render;

    #[test]
    fn terminal_output_has_product_and_sections() {
        let output = render(&collect_report(Profile::Balanced));
        assert!(output.contains("ExecLocus"));
        assert!(output.contains("CURRENT EXECUTION"));
        assert!(output.contains("TOOLCHAIN"));
    }
}
