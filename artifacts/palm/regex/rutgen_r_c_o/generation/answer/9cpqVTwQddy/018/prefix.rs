// Answer 0

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
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
    let mut ast = ast::ClassAscii {
        span: Span::new(0, 0),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };
    let _result = writer_instance.fmt_class_ascii(&ast);
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
    let mut printer = Printer { _priv: () };
    let mut ast = ast::ClassAscii {
        span: Span::new(0, 0),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };
    let _result = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alnum_not_negated() {
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
    let mut ast = ast::ClassAscii {
        span: Span::new(0, 0),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };
    let _result = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alnum_negated() {
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
    let mut ast = ast::ClassAscii {
        span: Span::new(0, 0),
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };
    let _result = writer_instance.fmt_class_ascii(&ast);
}

