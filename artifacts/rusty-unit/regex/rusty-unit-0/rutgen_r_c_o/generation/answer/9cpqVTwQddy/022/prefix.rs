// Answer 0

#[test]
fn test_fmt_class_ascii_blank_not_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
} 

#[test]
fn test_fmt_class_ascii_blank_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha_not_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_cntrl_not_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Cntrl,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_space_not_negated() {
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
    let mut printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    let _ = writer_instance.fmt_class_ascii(&ast);
}

