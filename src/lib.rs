pub mod adapters;
pub mod model;
pub mod privacy;
pub mod probes;
pub mod renderers;
pub mod rules;

use model::{Profile, Report};
use probes::context::{ProbeContext, SystemProbeContext};
use probes::executable::{ExecutableResolver, PathExecutableResolver};
use probes::process::{
    MAX_PROCESS_ANCESTRY, RuntimeIdentityInspector, SystemRuntimeIdentityInspector,
    snapshot_failure,
};

#[must_use]
pub fn collect_report(profile: Profile) -> Report {
    collect_report_with_components(
        &SystemProbeContext,
        &PathExecutableResolver,
        Some(&SystemRuntimeIdentityInspector),
        profile,
    )
}

#[must_use]
pub fn collect_report_with(context: &dyn ProbeContext, profile: Profile) -> Report {
    collect_report_with_components(context, &PathExecutableResolver, None, profile)
}

#[must_use]
pub fn collect_report_with_resolver(
    context: &dyn ProbeContext,
    resolver: &dyn ExecutableResolver,
    profile: Profile,
) -> Report {
    collect_report_with_components(context, resolver, None, profile)
}

fn collect_report_with_components(
    context: &dyn ProbeContext,
    resolver: &dyn ExecutableResolver,
    identity_inspector: Option<&dyn RuntimeIdentityInspector>,
    profile: Profile,
) -> Report {
    let mut identity_failures = Vec::new();
    let identity = identity_inspector.and_then(|inspector| {
        inspector.inspect(MAX_PROCESS_ANCESTRY).map_or_else(
            |error| {
                identity_failures.push(snapshot_failure(&error));
                None
            },
            Some,
        )
    });
    let runtime_result = probes::runtime::probe_with_identity(context, identity.as_ref());
    let runtime = runtime_result.value;
    let agent_result = adapters::probe(&runtime, identity.as_ref());
    let project_result = probes::path::probe_project_with(context, &runtime.kind);

    let mut evidence = runtime_result.evidence;
    evidence.extend(agent_result.evidence);
    evidence.extend(project_result.evidence);

    let mut probe_failures = identity_failures;
    probe_failures.extend(runtime_result.failures);
    probe_failures.extend(agent_result.failures);
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
        schema_version: "0.3.0".to_owned(),
        generated_at_unix_ms: context.now_unix_ms(),
        profile,
        runtime,
        agent: agent_result.value,
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
