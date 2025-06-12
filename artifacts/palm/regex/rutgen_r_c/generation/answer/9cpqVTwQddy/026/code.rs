// Answer 0

fn test_fmt_class_ascii_alpha_not_negated() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::ClassAscii {
        span: Span {}, // Construct a suitable span if necessary
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:alpha:]");
}

fn test_fmt_class_ascii_alpha_negated() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::ClassAscii {
        span: Span {}, // Construct a suitable span if necessary
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:^alpha:]");
}

