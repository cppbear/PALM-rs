// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };

    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span::default(), // assuming a default span for simplicity
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    });

    let result = writer.visit_class_set_item_post(&ast_item);

    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "[:alnum:]");
}

#[test]
fn test_visit_class_set_item_post_ascii_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };

    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span::default(), // assuming a default span for simplicity
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    });

    let result = writer.visit_class_set_item_post(&ast_item);

    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "[:^digit:]");
}

