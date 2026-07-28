use std::{cmp::Reverse, env};

use crate::{
    model::{ExecutableOrigin, PathClass, Report, Topology},
    rules,
};

const REDACTED_USER: &str = "[redacted-user]";
const REDACTED_MACHINE: &str = "[redacted-machine]";
const REDACTED_HOME: &str = "[redacted-home]";
const REDACTED_PATH: &str = "[redacted-path]";

pub trait RedactionContext {
    fn env_var(&self, key: &str) -> Option<String>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemRedactionContext;

impl RedactionContext for SystemRedactionContext {
    fn env_var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
}

#[derive(Clone)]
struct Secret {
    value: String,
    replacement: &'static str,
}

#[must_use]
pub fn redact_for_sharing(report: &Report) -> Report {
    redact_with_context(report, &SystemRedactionContext)
}

#[must_use]
pub fn redact_with_context(report: &Report, context: &dyn RedactionContext) -> Report {
    let secrets = collect_secrets(report, context);
    let mut redacted = report.clone();

    redacted.runtime.os_name = redact_free_text(&redacted.runtime.os_name, &secrets);
    redacted.runtime.distribution = redacted
        .runtime
        .distribution
        .as_deref()
        .map(|value| redact_free_text(value, &secrets));
    redacted.runtime.user = redacted
        .runtime
        .user
        .as_ref()
        .map(|_| REDACTED_USER.to_owned());
    redacted.runtime.shell = redacted
        .runtime
        .shell
        .as_deref()
        .map(|value| redact_free_text(value, &secrets));
    redacted.runtime.terminal = redacted
        .runtime
        .terminal
        .as_deref()
        .map(|value| redact_path_hint(value, &secrets));

    redacted.project.path = redacted
        .project
        .path
        .as_ref()
        .map(|_| project_placeholder(redacted.project.class).to_owned());

    for executable in &mut redacted.executables {
        executable.role = redact_text(&executable.role, &secrets);
        executable.requested = redact_path_hint(&executable.requested, &secrets);
        let selected_path = executable
            .selected
            .as_ref()
            .map(|candidate| candidate.path.clone());
        let selected_index = selected_path.as_ref().and_then(|path| {
            executable
                .candidates
                .iter()
                .position(|candidate| &candidate.path == path)
        });

        for (index, candidate) in executable.candidates.iter_mut().enumerate() {
            candidate.path = executable_placeholder(candidate.origin, &executable.role, index + 1);
        }
        executable.selected = selected_index
            .and_then(|index| executable.candidates.get(index).cloned())
            .or_else(|| {
                executable.selected.as_ref().map(|candidate| {
                    let mut candidate = candidate.clone();
                    candidate.path = executable_placeholder(candidate.origin, &executable.role, 1);
                    candidate
                })
            });
    }

    for evidence in &mut redacted.evidence {
        evidence.id = redact_text(&evidence.id, &secrets);
        evidence.probe = redact_text(&evidence.probe, &secrets);
        evidence.kind = redact_text(&evidence.kind, &secrets);
        evidence.claim = redact_free_text(&evidence.claim, &secrets);
        evidence.value = match (evidence.sensitive, evidence.id.as_str()) {
            (true, "runtime.user") => Some(REDACTED_USER.to_owned()),
            (true, "project.path") => redacted.project.path.clone(),
            (true, id) if id.starts_with("executable.") => redacted
                .executables
                .iter()
                .find(|executable| id == format!("executable.{}", executable.role))
                .and_then(|executable| executable.selected.as_ref())
                .map(|candidate| candidate.path.clone())
                .or_else(|| Some(REDACTED_PATH.to_owned())),
            (true, _) => evidence.value.as_ref().map(|_| "[redacted]".to_owned()),
            (false, _) => evidence
                .value
                .as_deref()
                .map(|value| redact_value(value, &secrets)),
        };
    }

    for failure in &mut redacted.probe_failures {
        failure.probe = redact_text(&failure.probe, &secrets);
        failure.code = redact_text(&failure.code, &secrets);
        "optional probe details omitted from shareable report".clone_into(&mut failure.message);
    }

    redacted.topology = Topology::from_report(&redacted);
    redacted.findings = rules::evaluate(&redacted);
    for finding in &mut redacted.findings {
        finding.title = redact_free_text(&finding.title, &secrets);
        finding.summary = redact_free_text(&finding.summary, &secrets);
        for action in &mut finding.suggested_actions {
            *action = redact_free_text(action, &secrets);
        }
    }

    redacted
}

fn collect_secrets(report: &Report, context: &dyn RedactionContext) -> Vec<Secret> {
    let mut secrets = Vec::new();
    if let Some(value) = &report.runtime.user {
        push_secret(&mut secrets, value, REDACTED_USER);
    }
    for key in ["USERNAME", "USER"] {
        if let Some(value) = context.env_var(key) {
            push_secret(&mut secrets, &value, REDACTED_USER);
        }
    }
    for key in ["USERPROFILE", "HOME"] {
        if let Some(value) = context.env_var(key) {
            push_secret(&mut secrets, &value, REDACTED_HOME);
        }
    }
    for key in ["COMPUTERNAME", "HOSTNAME"] {
        if let Some(value) = context.env_var(key) {
            push_secret(&mut secrets, &value, REDACTED_MACHINE);
        }
    }
    if let Some(value) = &report.project.path {
        push_secret(&mut secrets, value, REDACTED_PATH);
    }
    for path in report
        .executables
        .iter()
        .flat_map(|executable| &executable.candidates)
        .map(|candidate| &candidate.path)
    {
        push_secret(&mut secrets, path, REDACTED_PATH);
    }
    for value in report
        .evidence
        .iter()
        .filter_map(|evidence| evidence.value.as_deref())
        .filter(|value| is_absolute_path_like(value))
    {
        push_secret(&mut secrets, value, REDACTED_PATH);
    }
    secrets.sort_by_key(|secret| Reverse(secret.value.len()));
    secrets
}

fn push_secret(secrets: &mut Vec<Secret>, value: &str, replacement: &'static str) {
    let value = value.trim();
    if value.is_empty()
        || secrets
            .iter()
            .any(|secret| secret.value.eq_ignore_ascii_case(value))
    {
        return;
    }
    secrets.push(Secret {
        value: value.to_owned(),
        replacement,
    });
}

fn redact_text(value: &str, secrets: &[Secret]) -> String {
    secrets.iter().fold(value.to_owned(), |redacted, secret| {
        replace_ascii_case_insensitive(&redacted, &secret.value, secret.replacement)
    })
}

fn redact_free_text(value: &str, secrets: &[Secret]) -> String {
    let redacted = redact_text(value, secrets);
    if contains_absolute_path_token(&redacted) {
        "[redacted-text-containing-path]".to_owned()
    } else {
        redacted
    }
}

fn redact_value(value: &str, secrets: &[Secret]) -> String {
    let redacted = redact_text(value, secrets);
    if contains_absolute_path_token(&redacted) {
        REDACTED_PATH.to_owned()
    } else {
        redacted
    }
}

fn redact_path_hint(value: &str, secrets: &[Secret]) -> String {
    let redacted = redact_text(value, secrets);
    if is_absolute_path_like(&redacted) {
        redacted
            .rsplit(['/', '\\'])
            .find(|part| !part.is_empty())
            .map_or_else(|| REDACTED_PATH.to_owned(), str::to_owned)
    } else {
        redacted
    }
}

fn is_absolute_path_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    value.starts_with('/')
        || value.starts_with("\\\\")
        || value.starts_with("//")
        || (bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'/' | b'\\'))
}

fn contains_absolute_path_token(value: &str) -> bool {
    value.split_whitespace().any(|token| {
        let token = token.trim_matches(['"', '\'', '(', ')', '[', ']', '{', '}', ',', ';']);
        is_absolute_path_like(token)
            || token
                .split_once('=')
                .is_some_and(|(_, candidate)| is_absolute_path_like(candidate))
    })
}

const fn project_placeholder(class: PathClass) -> &'static str {
    match class {
        PathClass::WindowsNative => "[windows-project]",
        PathClass::WindowsMounted => "[windows-mounted-project]",
        PathClass::WslNative => "[wsl-project]",
        PathClass::WslUnc => "[wsl-unc-project]",
        PathClass::LinuxNative => "[linux-project]",
        PathClass::Unknown => REDACTED_PATH,
    }
}

fn executable_placeholder(origin: ExecutableOrigin, role: &str, index: usize) -> String {
    let origin = match origin {
        ExecutableOrigin::Windows => "windows",
        ExecutableOrigin::Linux => "linux",
        ExecutableOrigin::Script => "script",
        ExecutableOrigin::Unknown => "unknown",
    };
    format!("[{origin}-executable:{role}:{index}]")
}

fn replace_ascii_case_insensitive(value: &str, needle: &str, replacement: &str) -> String {
    if needle.is_empty() {
        return value.to_owned();
    }
    if !needle.is_ascii() {
        return value.replace(needle, replacement);
    }

    let lowercase = value.to_ascii_lowercase();
    let needle = needle.to_ascii_lowercase();
    let mut result = String::with_capacity(value.len());
    let mut cursor = 0;
    while let Some(offset) = lowercase[cursor..].find(&needle) {
        let start = cursor + offset;
        result.push_str(&value[cursor..start]);
        result.push_str(replacement);
        cursor = start + needle.len();
    }
    result.push_str(&value[cursor..]);
    result
}

#[cfg(test)]
mod tests {
    use super::replace_ascii_case_insensitive;

    #[test]
    fn replacement_is_ascii_case_insensitive() {
        assert_eq!(
            replace_ascii_case_insensitive(
                r"C:\\Users\\ALICE\\project",
                r"c:\\users\\alice",
                "[home]"
            ),
            r"[home]\\project"
        );
    }
}
