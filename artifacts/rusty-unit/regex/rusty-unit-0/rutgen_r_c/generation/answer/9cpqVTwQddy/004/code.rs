// Answer 0

#[test]
fn test_fmt_class_ascii_word() {
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

    let ast = ClassAscii {
        span: Span {}, // Assuming a valid Span instance is created, based on your context
        kind: ClassAsciiKind::Word,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "[:word:]");
}

#[test]
fn test_fmt_class_ascii_negated_word() {
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

    let ast = ClassAscii {
        span: Span {}, // Assuming a valid Span instance is created, based on your context
        kind: ClassAsciiKind::Word,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "[:^word:]");
}

#[test]
fn test_fmt_class_ascii_alphanumeric() {
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

    let ast = ClassAscii {
        span: Span {}, // Assuming a valid Span instance is created, based on your context
        kind: ClassAsciiKind::Alnum,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:alnum:]");
}

