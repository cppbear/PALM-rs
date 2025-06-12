// Answer 0

#[test]
fn test_visit_class_set_item_post_range() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let start_literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };

    let end_literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    };

    let range = ast::ClassSetRange {
        span: Span::default(),
        start: start_literal,
        end: end_literal,
    };

    let class_set_item = ast::ClassSetItem::Range(range);
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "a-z");
}

#[test]
fn test_visit_class_set_item_post_literal() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'b',
    };

    let class_set_item = ast::ClassSetItem::Literal(literal);
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "b");
}

#[test]
fn test_visit_class_set_item_post_empty() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let class_set_item = ast::ClassSetItem::Empty(Span::default());
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };

    let class_set_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_unity() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let class_set_item = ast::ClassSetItem::Union(ast::ClassSetUnion { /* Add details as necessary */ });
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert_eq!(result, Ok(()));
}

