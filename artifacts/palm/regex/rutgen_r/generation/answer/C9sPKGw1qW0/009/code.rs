// Answer 0

#[test]
fn test_visit_class_set_item_post_range() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                wtr: MockWriter::new(),
            }
        }

        fn fmt_literal(&mut self, _lit: &char) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let start_char = 'a';
    let end_char = 'z';

    let ast = ast::ClassSetItem::Range(Box::new(ast::Range {
        start: start_char,
        end: end_char,
    }));

    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

