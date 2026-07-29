pub(super) fn matches(process_name: &str) -> bool {
    normalize(process_name) == "claude"
}

fn normalize(process_name: &str) -> String {
    let normalized = process_name.trim().to_ascii_lowercase();
    normalized
        .strip_suffix(".exe")
        .unwrap_or(&normalized)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn requires_the_exact_visible_cli_name() {
        assert!(matches("claude"));
        assert!(matches("CLAUDE.EXE"));
        assert!(!matches("claude-desktop"));
        assert!(!matches("node"));
    }
}
