use std::process::Command;

#[test]
fn known_rule_is_explained_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_execlocus"))
        .args(["explain", "env002"])
        .output()
        .expect("execlocus should start");

    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("ENV002"));
}

#[test]
fn japanese_language_localizes_human_readable_explanation() {
    let output = Command::new(env!("CARGO_BIN_EXE_execlocus"))
        .args(["--lang", "ja", "explain", "env002"])
        .output()
        .expect("execlocus should start");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("WSL実行でWindows実行ファイルが選択される"));
    assert!(stdout.contains("このルールが必要な理由"));
    assert!(!stdout.contains("WHY THIS RULE EXISTS"));
}

#[test]
fn unknown_rule_exits_with_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_execlocus"))
        .args(["explain", "NOT_A_RULE"])
        .output()
        .expect("execlocus should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown rule ID"));
    assert!(output.stdout.is_empty());
}
