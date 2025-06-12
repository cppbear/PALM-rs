// Answer 0

#[test]
fn test_visit_class_set_item_post_range_fails_on_end() {
    use regex_syntax::ast;

    struct MockWriter {
        output: String,
        write_called: bool,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
                write_called: false,
            }
        }
        
        fn write_str(&mut self, s: &str) -> Result<(), ()> {
            self.output.push_str(s);
            self.write_called = true;
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_literal(&self, _: &ast::Literal) -> Result<(), ()> {
            Ok(())
        }
        
        fn fmt_class_ascii(&self, _: &ast::Ascii) -> Result<(), ()> {
            Ok(())
        }
        
        fn fmt_class_unicode(&self, _: &ast::Unicode) -> Result<(), ()> {
            Ok(())
        }
        
        fn fmt_class_perl(&self, _: &ast::Perl) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_bracketed_post(&self, _: &ast::Bracketed) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let start_literal = ast::Literal::from_char('a');
    let end_literal = ast::Literal::from_char('c');
    let range_item = ast::Range { start: start_literal, end: end_literal };
    let class_set_item = ast::ClassSetItem::Range(range_item);

    // Expect this to fail due to the error from fmt_literal on end.
    let result = visitor.visit_class_set_item_post(&class_set_item);
    assert!(result.is_err());
    assert!(visitor.wtr.write_called); // Check that write_str was called
}

