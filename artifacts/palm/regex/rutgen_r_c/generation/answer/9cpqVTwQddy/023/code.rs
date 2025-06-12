// Answer 0

#[test]
fn test_fmt_class_ascii_not_negated() {
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

    // Create a mock instance of the Printer
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast_not_negated = ClassAscii {
        span: Span { start: 0, end: 5 },
        kind: ClassAsciiKind::Ascii,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast_not_negated).unwrap();

    assert_eq!(writer.output, "[:ascii:]");
}

#[test]
fn test_fmt_class_ascii_negated() {
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

    // Create a mock instance of the Printer
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast_negated = ClassAscii {
        span: Span { start: 0, end: 5 },
        kind: ClassAsciiKind::Ascii,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast_negated).unwrap();

    assert_eq!(writer.output, "[:^ascii:]");
}

