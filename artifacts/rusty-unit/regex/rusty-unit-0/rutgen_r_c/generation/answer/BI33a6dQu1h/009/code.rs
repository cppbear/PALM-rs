// Answer 0

#[test]
fn test_fmt_flags_single_flag() {
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
    let mut printer = Printer { _priv: () };

    let flag = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            }
        ],
    };

    let mut my_writer = Writer { printer: &mut printer, wtr: &mut writer };
    let result = my_writer.fmt_flags(&flag);

    assert!(result.is_ok());
    assert_eq!(writer.output, "m");
}

#[test]
fn test_fmt_flags_multiple_flags() {
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
    let mut printer = Printer { _priv: () };

    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            },
        ],
    };

    let mut my_writer = Writer { printer: &mut printer, wtr: &mut writer };
    let result = my_writer.fmt_flags(&flags);

    assert!(result.is_ok());
    assert_eq!(writer.output, "im");
}

#[test]
fn test_fmt_flags_negation() {
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
    let mut printer = Printer { _priv: () };

    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::Unicode),
            },
        ],
    };

    let mut my_writer = Writer { printer: &mut printer, wtr: &mut writer };
    let result = my_writer.fmt_flags(&flags);

    assert!(result.is_ok());
    assert_eq!(writer.output, "-u");
}

#[test]
fn test_fmt_flags_empty_flags() {
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
    let mut printer = Printer { _priv: () };

    let flags = Flags {
        span: Span::default(),
        items: vec![],
    };

    let mut my_writer = Writer { printer: &mut printer, wtr: &mut writer };
    let result = my_writer.fmt_flags(&flags);

    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

