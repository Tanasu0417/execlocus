use std::path::PathBuf;

use crate::{
    model::{
        AgentInfo, AgentProduct, AgentStateKind, AgentStateLocation, Confidence, Evidence,
        ObservationStatus, ProbeResult, RuntimeKind,
    },
    probes::{context::ProbeContext, path::classify_path},
};

/// Derives the documented primary configuration root for the active agent.
///
/// The probe reads only allowlisted location variables and never opens files in
/// the configuration directory. A location is emitted only for an agent seen
/// through high-confidence process ancestry.
#[must_use]
pub fn probe_with(
    context: &dyn ProbeContext,
    agent: &AgentInfo,
) -> ProbeResult<Vec<AgentStateLocation>> {
    let Some(product) = agent.product else {
        return empty_result();
    };
    if agent.product_confidence != Confidence::High
        || !matches!(
            agent.runtime_confidence,
            Confidence::High | Confidence::Certain
        )
        || agent.runtime == RuntimeKind::Unknown
    {
        return empty_result();
    }

    let path = match product {
        AgentProduct::Codex => nonempty_env(context, "CODEX_HOME")
            .map(PathBuf::from)
            .or_else(|| home_dir(context, agent.runtime).map(|home| home.join(".codex"))),
        AgentProduct::ClaudeCode => {
            home_dir(context, agent.runtime).map(|home| home.join(".claude"))
        }
    };
    let Some(path) = path else {
        return empty_result();
    };

    let path = path.to_string_lossy().into_owned();
    let location = AgentStateLocation {
        product,
        kind: AgentStateKind::PrimaryConfig,
        class: classify_path(&path, agent.runtime),
        path: path.clone(),
        status: ObservationStatus::Inferred,
        confidence: Confidence::High,
    };
    let evidence_id = location.evidence_id();

    ProbeResult {
        value: vec![location],
        evidence: vec![Evidence {
            id: evidence_id,
            probe: "agent-state/v1".to_owned(),
            kind: "configuration-path".to_owned(),
            claim: "primary configuration root derived for the active agent without reading its contents"
                .to_owned(),
            value: Some(path),
            sensitive: true,
        }],
        failures: Vec::new(),
    }
}

fn home_dir(context: &dyn ProbeContext, runtime: RuntimeKind) -> Option<PathBuf> {
    let key = if runtime == RuntimeKind::WindowsNative {
        "USERPROFILE"
    } else {
        "HOME"
    };
    nonempty_env(context, key).map(PathBuf::from)
}

fn nonempty_env(context: &dyn ProbeContext, key: &str) -> Option<String> {
    context
        .env_var(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn empty_result() -> ProbeResult<Vec<AgentStateLocation>> {
    ProbeResult {
        value: Vec::new(),
        evidence: Vec::new(),
        failures: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        io,
        path::{Path, PathBuf},
    };

    use crate::{
        model::{AgentInfo, AgentProduct, Confidence, ObservationStatus, PathClass, RuntimeKind},
        probes::context::{CandidateSnapshot, HostPlatform, ProbeContext},
    };

    use super::probe_with;

    struct Context {
        environment: HashMap<String, String>,
    }

    impl ProbeContext for Context {
        fn host_platform(&self) -> HostPlatform {
            HostPlatform::Linux
        }

        fn os_name(&self) -> String {
            "linux".to_owned()
        }

        fn env_var(&self, key: &str) -> Option<String> {
            self.environment.get(key).cloned()
        }

        fn path_entries(&self) -> Vec<PathBuf> {
            Vec::new()
        }

        fn current_dir(&self) -> io::Result<PathBuf> {
            Ok(PathBuf::from("/fixture"))
        }

        fn inspect_candidate(
            &self,
            _directory: &Path,
            _name: &str,
            _prefix_limit: usize,
        ) -> io::Result<Option<CandidateSnapshot>> {
            Ok(None)
        }

        fn read_text(&self, _path: &Path, _max_bytes: usize) -> io::Result<String> {
            panic!("agent state probing must not read configuration contents")
        }

        fn now_unix_ms(&self) -> u128 {
            0
        }
    }

    fn agent(product: AgentProduct, confidence: Confidence) -> AgentInfo {
        AgentInfo {
            product: Some(product),
            product_status: ObservationStatus::Inferred,
            product_confidence: confidence,
            product_source: None,
            runtime: RuntimeKind::Wsl,
            runtime_status: ObservationStatus::Observed,
            runtime_confidence: Confidence::Certain,
            installations: Vec::new(),
            state_locations: Vec::new(),
        }
    }

    #[test]
    fn classifies_an_allowlisted_cross_layer_codex_home_without_reading_contents() {
        let context = Context {
            environment: HashMap::from([(
                "CODEX_HOME".to_owned(),
                "/mnt/c/fixture/.codex".to_owned(),
            )]),
        };
        let result = probe_with(&context, &agent(AgentProduct::Codex, Confidence::High));

        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].class, PathClass::WindowsMounted);
        assert_eq!(result.value[0].confidence, Confidence::High);
        assert!(result.evidence[0].sensitive);
    }

    #[test]
    fn derives_claude_root_from_home_only_for_high_confidence_process_evidence() {
        let context = Context {
            environment: HashMap::from([("HOME".to_owned(), "/home/fixture".to_owned())]),
        };
        let high = probe_with(&context, &agent(AgentProduct::ClaudeCode, Confidence::High));
        assert_eq!(high.value[0].class, PathClass::WslNative);

        let medium = probe_with(&context, &agent(AgentProduct::Codex, Confidence::Medium));
        assert!(medium.value.is_empty());
        assert!(medium.evidence.is_empty());
    }
}
