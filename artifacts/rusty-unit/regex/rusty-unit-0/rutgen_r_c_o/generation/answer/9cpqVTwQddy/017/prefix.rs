// Answer 0

#[test]
fn test_fmt_class_ascii_digit_negated() {
    struct MockWriter;
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = MockWriter;
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit_non_negated() {
    struct MockWriter;
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = MockWriter;
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast);
}

