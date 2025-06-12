// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alnum,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    
    assert_eq!(writer.output, "[:^alnum:]");
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alpha,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^alpha:]");
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Digit,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^digit:]");
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Upper,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^upper:]");
}

