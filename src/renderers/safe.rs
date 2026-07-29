use std::fmt::Write;

#[must_use]
pub(crate) fn terminal_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                write!(escaped, "\\u{{{:x}}}", u32::from(character))
                    .expect("writing to String cannot fail");
            }
            character => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::terminal_text;

    #[test]
    fn preserves_unicode_but_escapes_terminal_controls() {
        assert_eq!(
            terminal_text("日本語\u{1b}[31m\nnext\tvalue"),
            r"日本語\u{1b}[31m\nnext\tvalue"
        );
    }
}
