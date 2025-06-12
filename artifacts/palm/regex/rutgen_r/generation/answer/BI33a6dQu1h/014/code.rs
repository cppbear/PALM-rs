// Answer 0

#[test]
fn test_fmt_flags_with_negation() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation },
        ],
    };
    
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "-");
}

#[test]
fn test_fmt_flags_with_multiple_flags() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) },
        ],
    };
    
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "im");
}

#[test]
fn test_fmt_flags_with_ignore_whitespace_and_unicode() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) },
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
        ],
    };
    
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "xu");
}

#[test]
fn test_fmt_flags_with_empty_items() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let ast = ast::Flags { items: vec![] };
    
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "");
}

