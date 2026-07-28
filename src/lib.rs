pub mod model;
pub mod probes;
pub mod renderers;
pub mod rules;

use model::{Profile, Report};
use probes::context::{ProbeContext, SystemProbeContext};
use probes::executable::{ExecutableResolver, PathExecutableResolver};

#[must_use]
pub fn collect_report(profile: Profile) -> Report {
    collect_report_with(&SystemProbeContext, profile)
}

#[must_use]
pub fn collect_report_with(context: &dyn ProbeContext, profile: Profile) -> Report {
    collect_report_with_resolver(context, &PathExecutableResolver, profile)
}

#[must_use]
pub fn collect_report_with_resolver(
    context: &dyn ProbeContext,
    resolver: &dyn ExecutableResolver,
    profile: Profile,
) -> Report {
    let runtime_result = probes::runtime::probe_with(context);
    let runtime = runtime_result.value;
    let project_result = probes::path::probe_project_with(context, &runtime.kind);

    let mut evidence = runtime_result.evidence;
    evidence.extend(project_result.evidence);

    let mut probe_failures = runtime_result.failures;
    probe_failures.extend(project_result.failures);

    let mut executables = Vec::new();
    for (role, command) in [("git", "git"), ("node", "node"), ("npm", "npm")] {
        let result = probes::executable::probe_with_resolver(
            context,
            resolver,
            role,
            command,
            &runtime.kind,
        );
        evidence.extend(result.evidence);
        probe_failures.extend(result.failures);
        executables.push(result.value);
    }

    let mut report = Report {
        schema_version: "0.1.0".to_owned(),
        generated_at_unix_ms: context.now_unix_ms(),
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
