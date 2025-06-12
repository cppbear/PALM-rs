// Answer 0

#[test]
fn test_fmt_flags_case_insensitive() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let flags_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
    };

    let flags = ast::Flags {
        span: Span::default(),
        items: vec![flags_item],
    };

    let mut writer_struct = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    writer_struct.fmt_flags(&flags).unwrap();
    assert_eq!(writer_struct.wtr.output, "i");
}

#[test]
fn test_fmt_flags_multiple_flags() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let flags_items = vec![
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };

    let mut writer_struct = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    writer_struct.fmt_flags(&flags).unwrap();
    assert_eq!(writer_struct.wtr.output, "ims");
}

#[test]
fn test_fmt_flags_negation() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let flags_items = vec![
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Negation,
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };

    let mut writer_struct = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    writer_struct.fmt_flags(&flags).unwrap();
    assert_eq!(writer_struct.wtr.output, "-u");
}

#[test]
#[should_panic]
fn test_fmt_flags_error_case() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // Simulating an error condition
            Err(fmt::Error)
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let flags_items = vec![ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
    }];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };

    let mut writer_struct = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    writer_struct.fmt_flags(&flags).unwrap();
}

