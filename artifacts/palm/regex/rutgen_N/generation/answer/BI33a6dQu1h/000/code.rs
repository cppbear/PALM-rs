// Answer 0

struct TestWriter {
    output: String,
}

impl TestWriter {
    fn new() -> Self {
        TestWriter { output: String::new() }
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

#[test]
fn test_fmt_flags_case_insensitive() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "i");
}

#[test]
fn test_fmt_flags_multi_line() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "m");
}

#[test]
fn test_fmt_flags_dot_matches_new_line() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "s");
}

#[test]
fn test_fmt_flags_swap_greed() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "U");
}

#[test]
fn test_fmt_flags_unicode() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "u");
}

#[test]
fn test_fmt_flags_ignore_whitespace() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "x");
}

#[test]
fn test_fmt_flags_negation() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Negation }],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "-");
}

#[test]
fn test_fmt_flags_combined() {
    let mut writer = TestWriter::new();
    let mut flags = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
        ],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "-iu");
}

