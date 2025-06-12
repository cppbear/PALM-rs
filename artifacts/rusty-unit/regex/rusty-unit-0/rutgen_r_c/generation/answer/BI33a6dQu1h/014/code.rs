// Answer 0

#[test]
fn test_fmt_flags_with_single_negation() {
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
    
    let flags_item = FlagsItem {
        span: Span::default(),
        kind: FlagsItemKind::Negation,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![flags_item],
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };
    let result = writer_ref.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "-");
}

#[test]
fn test_fmt_flags_with_multiple_flags() {
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

    let flags_items = vec![
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
            kind: FlagsItemKind::Flag(Flag::Unicode),
        },
    ];

    let flags = Flags {
        span: Span::default(),
        items: flags_items,
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };
    let result = writer_ref.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "imu");
}

#[test]
fn test_fmt_flags_with_negation_and_flags() {
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

    let flags_items = vec![
        FlagsItem {
            span: Span::default(),
            kind: FlagsItemKind::Negation,
        },
        FlagsItem {
            span: Span::default(),
            kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
        },
    ];

    let flags = Flags {
        span: Span::default(),
        items: flags_items,
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };
    let result = writer_ref.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "-x");
}

#[test]
fn test_fmt_flags_with_empty_items() {
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
        items: Vec::new(),
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };
    let result = writer_ref.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

