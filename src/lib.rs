pub mod adapters;
pub mod model;
pub mod privacy;
pub mod probes;
pub mod renderers;
pub mod rules;

use model::{Evidence, Profile, Report};
use probes::context::{ProbeContext, SystemProbeContext};
use probes::executable::{ExecutableResolver, PathExecutableResolver};
use probes::process::{
    MAX_PROCESS_ANCESTRY, RuntimeIdentityInspector, SystemRuntimeIdentityInspector,
    snapshot_failure,
};
use probes::shell::{ShellKind, ShellSessionSnapshot};

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
    let codex_thread_id = context.env_var("CODEX_THREAD_ID");
    let mut agent_result = adapters::probe_with_codex_thread_id(
        &runtime,
        identity.as_ref(),
        codex_thread_id.as_deref(),
    );
    let mut installation_evidence = Vec::new();
    let mut installation_failures = Vec::new();
    for product in [model::AgentProduct::Codex, model::AgentProduct::ClaudeCode] {
        let result = probes::agent_installation::probe_with(context, product, runtime.kind);
        installation_evidence.extend(result.evidence);
        installation_failures.extend(result.failures);
        agent_result.value.installations.push(result.value);
    }
    let agent_state_result = probes::agent_state::probe_with(context, &agent_result.value);
    agent_result.value.state_locations = agent_state_result.value;
    let project_result = probes::path::probe_project_with(context, &runtime.kind);

    let mut evidence = vec![Evidence {
        id: "profile.selected".to_owned(),
        probe: "cli/v1".to_owned(),
        kind: "configuration".to_owned(),
        claim: "selected diagnostic profile".to_owned(),
        value: Some(profile.label().to_owned()),
        sensitive: false,
    }];
    evidence.extend(runtime_result.evidence);
    evidence.extend(agent_result.evidence);
    evidence.extend(installation_evidence);
    evidence.extend(agent_state_result.evidence);
    evidence.extend(project_result.evidence);

    let mut probe_failures = identity_failures;
    probe_failures.extend(runtime_result.failures);
    probe_failures.extend(agent_result.failures);
    probe_failures.extend(installation_failures);
    probe_failures.extend(agent_state_result.failures);
    probe_failures.extend(project_result.failures);

    let mut executables = Vec::new();
    let active_shell = active_shell_contract(&runtime);
    for (role, command) in [
        ("codex", "codex"),
        ("claude", "claude"),
        ("git", "git"),
        ("node", "node"),
        ("npm", "npm"),
    ] {
        let result = active_shell.map_or_else(
            || {
                probes::executable::probe_with_resolver(
                    context,
                    resolver,
                    role,
                    command,
                    &runtime.kind,
                )
            },
            |shell| {
                probes::executable::probe_with_shell_snapshot(
                    context,
                    role,
                    command,
                    &runtime.kind,
                    shell,
                    &ShellSessionSnapshot::unavailable(),
                )
            },
        );
        evidence.extend(result.evidence);
        probe_failures.extend(result.failures);
        executables.push(result.value);
    }

    let mut report = Report {
        schema_version: "0.5.0".to_owned(),
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

fn active_shell_contract(runtime: &model::RuntimeInfo) -> Option<ShellKind> {
    if runtime.shell_source != Some(model::RuntimeValueSource::ProcessAncestry) {
        return None;
    }
    runtime
        .shell
        .as_deref()
        .and_then(ShellKind::from_runtime_label)
}

#[cfg(test)]
mod tests {
    use crate::{
        active_shell_contract,
        model::{Confidence, ObservationStatus, RuntimeInfo, RuntimeKind, RuntimeValueSource},
        probes::shell::ShellKind,
    };

    fn runtime(shell: Option<&str>, source: Option<RuntimeValueSource>) -> RuntimeInfo {
        RuntimeInfo {
            kind: RuntimeKind::Wsl,
            kind_source: Some(RuntimeValueSource::KernelRelease),
            os_name: "WSL".to_owned(),
            distribution: Some("Synthetic Linux".to_owned()),
            distribution_source: Some(RuntimeValueSource::OsRelease),
            user: None,
            user_source: None,
            shell: shell.map(str::to_owned),
            shell_source: source,
            terminal: None,
            terminal_layer: RuntimeKind::Unknown,
            terminal_layer_status: ObservationStatus::Unavailable,
            terminal_layer_confidence: Confidence::None,
            terminal_layer_source: None,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        }
    }

    #[test]
    fn production_shell_contract_requires_process_ancestry_evidence() {
        assert_eq!(
            active_shell_contract(&runtime(
                Some("bash"),
                Some(RuntimeValueSource::ProcessAncestry)
            )),
            Some(ShellKind::Bash)
        );
        assert_eq!(
            active_shell_contract(&runtime(
                Some("bash"),
                Some(RuntimeValueSource::Environment)
            )),
            None
        );
        assert_eq!(
            active_shell_contract(&runtime(
                Some("unsupported-shell"),
                Some(RuntimeValueSource::ProcessAncestry)
            )),
            None
        );
    }
}
