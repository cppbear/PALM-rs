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
    let printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ClassAscii { 
        span: Span, // Assuming Span is initialized correctly as needed
        kind: ClassAsciiKind::Ascii, 
        negated: false 
    };

    writer_instance.fmt_class_ascii(&ast);
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
    let printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ClassAscii { 
        span: Span, // Assuming Span is initialized correctly as needed
        kind: ClassAsciiKind::Ascii, 
        negated: true 
    };

    writer_instance.fmt_class_ascii(&ast);
}

