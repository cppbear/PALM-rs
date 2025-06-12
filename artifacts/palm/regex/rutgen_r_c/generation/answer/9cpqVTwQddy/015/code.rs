// Answer 0

#[test]
fn test_fmt_class_ascii_graph_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let class_ascii = ClassAscii {
        span: Span::default(), // Assuming a default method exists for Span
        kind: ClassAsciiKind::Graph,
        negated: true,
    };

    let result = writer_instance.fmt_class_ascii(&class_ascii);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "[:^graph:]");
}

#[test]
fn test_fmt_class_ascii_graph_non_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let class_ascii = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Graph,
        negated: false,
    };

    let result = writer_instance.fmt_class_ascii(&class_ascii);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "[:graph:]");
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let class_ascii = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Digit,
        negated: true,
    };

    let result = writer_instance.fmt_class_ascii(&class_ascii);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "[:^digit:]");
}

#[test]
fn test_fmt_class_ascii_space_non_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let class_ascii = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    let result = writer_instance.fmt_class_ascii(&class_ascii);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "[:space:]");
}

