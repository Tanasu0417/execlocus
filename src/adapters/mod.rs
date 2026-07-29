mod claude;
mod codex;

use crate::{
    model::{
        AgentEvidenceSource, AgentInfo, AgentProduct, Confidence, Evidence, ObservationStatus,
        ProbeResult, RuntimeInfo, RuntimeKind,
    },
    probes::process::{MAX_PROCESS_ANCESTRY, RuntimeIdentitySnapshot},
};

/// Infers a supported agent product only from the bounded parent-process chain.
///
/// Installation presence, environment variables, and similar process names are
/// deliberately insufficient. The first record is skipped because the process
/// snapshot contract identifies it as `ExecLocus` itself.
#[must_use]
pub fn probe(
    runtime: &RuntimeInfo,
    identity: Option<&RuntimeIdentitySnapshot>,
) -> ProbeResult<AgentInfo> {
    let product = identity
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

    let Some(product) = product else {
        return ProbeResult {
            value: AgentInfo::default(),
            evidence: vec![Evidence {
                id: "agent.product".to_owned(),
                probe: "agent-process/v1".to_owned(),
                kind: "process".to_owned(),
                claim: "no supported agent product was observed in bounded process ancestry"
                    .to_owned(),
                value: None,
                sensitive: false,
            }],
            failures: Vec::new(),
        };
    };

    let runtime_is_available = runtime.kind != RuntimeKind::Unknown;
    let agent = AgentInfo {
        product: Some(product),
        product_status: ObservationStatus::Inferred,
        product_confidence: Confidence::High,
        product_source: Some(AgentEvidenceSource::ProcessAncestry),
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
    };

    let mut evidence = vec![Evidence {
        id: "agent.product".to_owned(),
        probe: "agent-process/v1".to_owned(),
        kind: "process".to_owned(),
        claim: "supported agent product inferred from an exact process name in bounded ancestry"
            .to_owned(),
        value: Some(product.evidence_value().to_owned()),
        sensitive: false,
    }];
    if runtime_is_available {
        evidence.push(Evidence {
            id: "agent.runtime".to_owned(),
            probe: "agent-process/v1".to_owned(),
            kind: "process-runtime".to_owned(),
            claim:
                "agent ancestor and current ExecLocus process share the observed OS process layer"
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

#[cfg(test)]
mod tests {
    use crate::{
        adapters::probe,
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
}
