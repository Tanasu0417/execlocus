mod claude;
mod codex;

use crate::{
    model::{
        AgentEvidenceSource, AgentInfo, AgentProduct, Confidence, Evidence, ObservationStatus,
        ProbeResult, RuntimeInfo, RuntimeKind,
    },
    probes::process::{MAX_PROCESS_ANCESTRY, RuntimeIdentitySnapshot},
};

struct ProductInference {
    product: AgentProduct,
    confidence: Confidence,
    source: AgentEvidenceSource,
    probe: &'static str,
    kind: &'static str,
    claim: &'static str,
}

/// Infers a supported agent product from the bounded parent-process chain.
///
/// Installation presence, arbitrary environment variables, and similar process
/// names are deliberately insufficient. The first record is skipped because
/// the process snapshot contract identifies it as `ExecLocus` itself.
#[must_use]
pub fn probe(
    runtime: &RuntimeInfo,
    identity: Option<&RuntimeIdentitySnapshot>,
) -> ProbeResult<AgentInfo> {
    probe_with_codex_thread_id(runtime, identity, None)
}

/// Adds a conservative Codex child-process marker fallback for sandboxes that
/// hide the launching agent behind a PID namespace.
///
/// The marker value is checked for UUID shape only and is never retained in the
/// report or evidence. Process ancestry remains stronger and always wins.
#[must_use]
pub fn probe_with_codex_thread_id(
    runtime: &RuntimeInfo,
    identity: Option<&RuntimeIdentitySnapshot>,
    codex_thread_id: Option<&str>,
) -> ProbeResult<AgentInfo> {
    let codex_thread_id = codex_thread_id
        .filter(|_| matches!(runtime.kind, RuntimeKind::Wsl | RuntimeKind::LinuxNative));
    let Some(inference) = infer_product(identity, codex_thread_id) else {
        return ProbeResult {
            value: AgentInfo::default(),
            evidence: vec![Evidence {
                id: "agent.product".to_owned(),
                probe: "agent-process/v1".to_owned(),
                kind: "process".to_owned(),
                claim: "no supported agent product was observed in bounded process ancestry or an allowlisted child-process marker".to_owned(),
                value: None,
                sensitive: false,
            }],
            failures: Vec::new(),
        };
    };

    let product = inference.product;
    let product_source = inference.source;
    let probe = inference.probe;

    let runtime_is_available = runtime.kind != RuntimeKind::Unknown;
    let agent = AgentInfo {
        product: Some(product),
        product_status: ObservationStatus::Inferred,
        product_confidence: inference.confidence,
        product_source: Some(product_source),
        runtime: if runtime_is_available {
            runtime.kind
        } else {
            RuntimeKind::Unknown
        },
        runtime_status: if runtime_is_available {
            runtime.status
        } else {
            ObservationStatus::Unavailable
        },
        runtime_confidence: if runtime_is_available {
            runtime.confidence
        } else {
            Confidence::None
        },
        installations: Vec::new(),
        state_locations: Vec::new(),
    };

    let mut evidence = vec![Evidence {
        id: "agent.product".to_owned(),
        probe: probe.to_owned(),
        kind: inference.kind.to_owned(),
        claim: inference.claim.to_owned(),
        value: Some(product.evidence_value().to_owned()),
        sensitive: false,
    }];
    if runtime_is_available {
        evidence.push(Evidence {
            id: "agent.runtime".to_owned(),
            probe: probe.to_owned(),
            kind: "process-runtime".to_owned(),
            claim: match product_source {
                AgentEvidenceSource::ProcessAncestry => {
                    "agent ancestor and current ExecLocus process share the observed OS process layer"
                }
                AgentEvidenceSource::EnvironmentMarker => {
                    "agent child-process marker and current ExecLocus process share the observed OS process layer"
                }
            }
            .to_owned(),
            value: Some(format!("{:?}", runtime.kind)),
            sensitive: false,
        });
    }

    ProbeResult {
        value: agent,
        evidence,
        failures: Vec::new(),
    }
}

fn infer_product(
    identity: Option<&RuntimeIdentitySnapshot>,
    codex_thread_id: Option<&str>,
) -> Option<ProductInference> {
    let process_product = identity
        .into_iter()
        .flat_map(|snapshot| {
            snapshot
                .process_ancestry
                .iter()
                .skip(1)
                .take(MAX_PROCESS_ANCESTRY.saturating_sub(1))
        })
        .find_map(|process| {
            codex::matches(&process.name)
                .then_some(AgentProduct::Codex)
                .or_else(|| claude::matches(&process.name).then_some(AgentProduct::ClaudeCode))
        });

    process_product
        .map(|product| ProductInference {
            product,
            confidence: Confidence::High,
            source: AgentEvidenceSource::ProcessAncestry,
            probe: "agent-process/v1",
            kind: "process",
            claim: "supported agent product inferred from an exact process name in bounded ancestry",
        })
        .or_else(|| {
            codex_thread_id
                .filter(|value| is_hyphenated_uuid(value))
                .map(|_| ProductInference {
                    product: AgentProduct::Codex,
                    confidence: Confidence::Medium,
                    source: AgentEvidenceSource::EnvironmentMarker,
                    probe: "agent-environment/v1",
                    kind: "environment",
                    claim: "Codex child-process marker observed with the expected UUID shape",
                })
        })
}

fn is_hyphenated_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }

    bytes.iter().enumerate().all(|(index, byte)| {
        if matches!(index, 8 | 13 | 18 | 23) {
            *byte == b'-'
        } else {
            byte.is_ascii_hexdigit()
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        adapters::{probe, probe_with_codex_thread_id},
        model::{
            AgentEvidenceSource, AgentProduct, Confidence, ObservationStatus, RuntimeInfo,
            RuntimeKind, RuntimeValueSource,
        },
        probes::process::{MAX_PROCESS_ANCESTRY, ProcessRecord, RuntimeIdentitySnapshot},
    };

    fn runtime(kind: RuntimeKind) -> RuntimeInfo {
        RuntimeInfo {
            kind,
            kind_source: Some(RuntimeValueSource::TargetPlatform),
            os_name: "fixture".to_owned(),
            distribution: None,
            distribution_source: None,
            user: None,
            user_source: None,
            shell: None,
            shell_source: None,
            terminal: None,
            terminal_layer: RuntimeKind::Unknown,
            terminal_layer_status: ObservationStatus::Unavailable,
            terminal_layer_confidence: Confidence::None,
            terminal_layer_source: None,
            status: ObservationStatus::Observed,
            confidence: Confidence::Certain,
        }
    }

    fn identity(names: &[&str]) -> RuntimeIdentitySnapshot {
        RuntimeIdentitySnapshot {
            user: None,
            process_ancestry: names
                .iter()
                .enumerate()
                .map(|(index, name)| ProcessRecord {
                    pid: u32::try_from(index + 1).expect("fixture PID fits u32"),
                    parent_pid: u32::try_from(index + 2).expect("fixture PID fits u32"),
                    name: (*name).to_owned(),
                })
                .collect(),
        }
    }

    #[test]
    fn detects_exact_codex_windows_process_name() {
        let snapshot = identity(&["execlocus.exe", "PWSH.EXE", "CODEX.EXE"]);
        let result = probe(&runtime(RuntimeKind::WindowsNative), Some(&snapshot));

        assert_eq!(result.value.product, Some(AgentProduct::Codex));
        assert_eq!(result.value.runtime, RuntimeKind::WindowsNative);
        assert_eq!(result.value.product_status, ObservationStatus::Inferred);
        assert_eq!(result.value.product_confidence, Confidence::High);
        assert_eq!(
            result.value.product_source,
            Some(AgentEvidenceSource::ProcessAncestry)
        );
    }

    #[test]
    fn detects_exact_claude_linux_process_name() {
        let snapshot = identity(&["execlocus", "bash", "claude"]);
        let result = probe(&runtime(RuntimeKind::Wsl), Some(&snapshot));

        assert_eq!(result.value.product, Some(AgentProduct::ClaudeCode));
        assert_eq!(result.value.runtime, RuntimeKind::Wsl);
    }

    #[test]
    fn nearest_supported_ancestor_wins() {
        let snapshot = identity(&["execlocus", "claude", "codex"]);
        let result = probe(&runtime(RuntimeKind::LinuxNative), Some(&snapshot));

        assert_eq!(result.value.product, Some(AgentProduct::ClaudeCode));
    }

    #[test]
    fn similar_names_and_wrapper_only_processes_remain_unknown() {
        for process in ["node", "codex-helper", "claude-desktop", "my-claude.exe"] {
            let snapshot = identity(&["execlocus", process]);
            let result = probe(&runtime(RuntimeKind::Wsl), Some(&snapshot));

            assert_eq!(result.value.product, None, "matched {process}");
            assert_eq!(result.value.runtime, RuntimeKind::Unknown);
            assert_eq!(result.value.product_status, ObservationStatus::Unavailable);
        }
    }

    #[test]
    fn current_executable_name_is_not_agent_evidence() {
        let snapshot = identity(&["codex.exe", "pwsh.exe"]);
        let result = probe(&runtime(RuntimeKind::WindowsNative), Some(&snapshot));

        assert_eq!(result.value.product, None);
    }

    #[test]
    fn missing_process_evidence_is_unknown_without_failure() {
        let result = probe(&runtime(RuntimeKind::Wsl), None);

        assert_eq!(result.value.product, None);
        assert_eq!(result.value.runtime, RuntimeKind::Unknown);
        assert!(result.failures.is_empty());
    }

    #[test]
    fn product_can_be_inferred_while_runtime_remains_unknown() {
        let snapshot = identity(&["execlocus", "codex"]);
        let result = probe(&runtime(RuntimeKind::Unknown), Some(&snapshot));

        assert_eq!(result.value.product, Some(AgentProduct::Codex));
        assert_eq!(result.value.product_status, ObservationStatus::Inferred);
        assert_eq!(result.value.runtime, RuntimeKind::Unknown);
        assert_eq!(result.value.runtime_status, ObservationStatus::Unavailable);
    }

    #[test]
    fn ignores_supported_name_beyond_the_process_depth_limit() {
        let mut names = vec!["execlocus"; MAX_PROCESS_ANCESTRY];
        names.push("codex");
        let snapshot = identity(&names);
        let result = probe(&runtime(RuntimeKind::WindowsNative), Some(&snapshot));

        assert_eq!(result.value.product, None);
    }

    #[test]
    fn codex_thread_marker_survives_a_hidden_parent_process_namespace() {
        let marker = "01234567-89ab-cdef-8123-456789abcdef";
        let snapshot = identity(&["execlocus", "3"]);
        let result =
            probe_with_codex_thread_id(&runtime(RuntimeKind::Wsl), Some(&snapshot), Some(marker));

        assert_eq!(result.value.product, Some(AgentProduct::Codex));
        assert_eq!(result.value.product_confidence, Confidence::Medium);
        assert_eq!(
            result.value.product_source,
            Some(AgentEvidenceSource::EnvironmentMarker)
        );
        assert!(result.evidence.iter().all(|evidence| {
            evidence.value.as_deref() != Some(marker) && !evidence.claim.contains(marker)
        }));
    }

    #[test]
    fn malformed_codex_thread_marker_is_not_product_evidence() {
        let snapshot = identity(&["execlocus", "3"]);
        let result = probe_with_codex_thread_id(
            &runtime(RuntimeKind::Wsl),
            Some(&snapshot),
            Some("not-a-thread-id"),
        );

        assert_eq!(result.value.product, None);
    }

    #[test]
    fn codex_thread_marker_is_ignored_outside_linux_and_wsl() {
        let marker = "01234567-89ab-cdef-8123-456789abcdef";
        let snapshot = identity(&["execlocus", "3"]);
        let result = probe_with_codex_thread_id(
            &runtime(RuntimeKind::WindowsNative),
            Some(&snapshot),
            Some(marker),
        );

        assert_eq!(result.value.product, None);
    }

    #[test]
    fn process_ancestry_wins_over_the_codex_thread_marker() {
        let marker = "01234567-89ab-cdef-8123-456789abcdef";
        let snapshot = identity(&["execlocus", "claude"]);
        let result =
            probe_with_codex_thread_id(&runtime(RuntimeKind::Wsl), Some(&snapshot), Some(marker));

        assert_eq!(result.value.product, Some(AgentProduct::ClaudeCode));
        assert_eq!(result.value.product_confidence, Confidence::High);
        assert_eq!(
            result.value.product_source,
            Some(AgentEvidenceSource::ProcessAncestry)
        );
    }
}
