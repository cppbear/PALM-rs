// Answer 0

#[test]
fn test_visit_class_set_item_post_range_success() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> Result<(), ()> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn fmt_literal(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }
        
        fn fmt_class_ascii(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_perl(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };

    let range_item = ast::ClassSetItem::Range(ast::Range { 
        start: "a".to_string(), 
        end: "z".to_string() 
    });

    let result = visitor.visit_class_set_item_post(&range_item);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_class_set_item_post_range_fmt_literal_err() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn fmt_literal(&mut self, _: &str) -> Result<(), ()> {
            Err(())
        }

        fn fmt_class_ascii(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_unicode(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_perl(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_bracketed_post(&mut self, _: &str) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };

    let range_item = ast::ClassSetItem::Range(ast::Range {
        start: "a".to_string(),
        end: "z".to_string(),
    });

    // This should panic due to an error in fmt_literal
    let _ = visitor.visit_class_set_item_post(&range_item);
}

