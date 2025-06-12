// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(), // Assuming Span::empty() exists
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^alnum:]");
}

#[test]
fn test_fmt_class_ascii_alnum_non_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:alnum:]");
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^alpha:]");
}

#[test]
fn test_fmt_class_ascii_alpha_non_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:alpha:]");
}

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(),
        kind: ast::ClassAsciiKind::Cntrl,
        negated: true,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^cntrl:]");
}

#[test]
fn test_fmt_class_ascii_cntrl_non_negated() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::empty(),
        kind: ast::ClassAsciiKind::Cntrl,
        negated: false,
    };

    writer_ref.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:cntrl:]");
}

