use crate::{
    model::{
        AgentInstallationInfo, AgentProduct, Confidence, Evidence, ObservationStatus, ProbeResult,
        RuntimeKind,
    },
    probes::{
        context::ProbeContext,
        executable::{resolve_from_search_plan, windows_pathext_candidate_names},
    },
};

/// Finds supported-agent executables in the current PATH directories without
/// treating installation presence as evidence of the active agent process.
#[must_use]
pub fn probe_with(
    context: &dyn ProbeContext,
    product: AgentProduct,
    runtime: RuntimeKind,
) -> ProbeResult<AgentInstallationInfo> {
    let command = match product {
        AgentProduct::Codex => "codex",
        AgentProduct::ClaudeCode => "claude",
    };
    let names = installation_candidate_names(context, command, runtime);
    let result =
        resolve_from_search_plan(context, command, runtime, context.path_entries(), &names);
    let status = match (result.value.is_empty(), result.failures.is_empty()) {
        (false, _) => ObservationStatus::Observed,
        (true, true) => ObservationStatus::Unavailable,
        (true, false) => ObservationStatus::Failed,
    };
    let confidence = match (result.value.is_empty(), result.failures.is_empty()) {
        (false, true) => Confidence::Certain,
        (false, false) => Confidence::High,
        (true, _) => Confidence::None,
    };
    let evidence = result
        .value
        .iter()
        .enumerate()
        .map(|(index, candidate)| Evidence {
            id: format!(
                "agent.installation.{}.candidate.{}",
                product.evidence_value(),
                index + 1
            ),
            probe: "agent-installation/v1".to_owned(),
            kind: "executable-candidate".to_owned(),
            claim: format!(
                "{} installation candidate {} has {:?} origin",
                product.label(),
                index + 1,
                candidate.origin
            ),
            value: Some(candidate.path.clone()),
            sensitive: true,
        })
        .collect();

    ProbeResult {
        value: AgentInstallationInfo {
            product,
            candidates: result.value,
            status,
            confidence,
        },
        evidence,
        failures: result.failures,
    }
}

fn installation_candidate_names(
    context: &dyn ProbeContext,
    command: &str,
    runtime: RuntimeKind,
) -> Vec<String> {
    if runtime == RuntimeKind::WindowsNative {
        windows_pathext_candidate_names(context, command, true)
    } else {
        ["", ".exe", ".cmd", ".bat"]
            .into_iter()
            .map(|extension| format!("{command}{extension}"))
            .collect()
    }
}
