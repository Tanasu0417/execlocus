pub mod model;
pub mod probes;
pub mod renderers;
pub mod rules;

use std::time::{SystemTime, UNIX_EPOCH};

use model::{Profile, Report};

#[must_use]
pub fn collect_report(profile: Profile) -> Report {
    let runtime_result = probes::runtime::probe();
    let runtime = runtime_result.value;
    let project_result = probes::path::probe_project(&runtime.kind);

    let mut evidence = runtime_result.evidence;
    evidence.extend(project_result.evidence);

    let mut probe_failures = runtime_result.failures;
    probe_failures.extend(project_result.failures);

    let mut executables = Vec::new();
    for (role, command) in [("git", "git"), ("node", "node"), ("npm", "npm")] {
        let result = probes::executable::probe(role, command, &runtime.kind);
        evidence.extend(result.evidence);
        probe_failures.extend(result.failures);
        executables.push(result.value);
    }

    let generated_at_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis());

    let mut report = Report {
        schema_version: "0.1.0".to_owned(),
        generated_at_unix_ms,
        profile,
        runtime,
        project: project_result.value,
        executables,
        topology: model::Topology::default(),
        evidence,
        findings: Vec::new(),
        probe_failures,
    };

    report.topology = model::Topology::from_report(&report);
    report.findings = rules::evaluate(&report);
    report
}
