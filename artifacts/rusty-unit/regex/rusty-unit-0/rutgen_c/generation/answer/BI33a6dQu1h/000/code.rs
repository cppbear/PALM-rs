// Answer 0

#[test]
fn test_fmt_flags() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let flags_item1 = ast::FlagsItem {
        span: ast::Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
    };

    let flags_item2 = ast::FlagsItem {
        span: ast::Span::default(),
        kind: ast::FlagsItemKind::Negation,
    };

    let flags = ast::Flags {
        span: ast::Span::default(),
        items: vec![flags_item1, flags_item2],
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = writer_instance.fmt_flags(&flags);
    assert!(result.is_ok());
    assert_eq!(writer.output, "i-");
}

#[test]
fn test_fmt_flags_empty() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let flags = ast::Flags {
        span: ast::Span::default(),
        items: Vec::new(),
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = writer_instance.fmt_flags(&flags);
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

