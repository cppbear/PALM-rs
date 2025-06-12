// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    struct TestVisitor {
        result: Result<(), ()>,
    }

    impl TestVisitor {
        fn fmt_literal(&self, _literal: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_ascii(&self, _ascii: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_unicode(&self, _unicode: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_perl(&self, _perl: &str) -> Result<(), ()> {
            Ok(())
        }

        fn fmt_class_bracketed_post(&self, _bracketed: &str) -> Result<(), ()> {
            Ok(())
        }
    }

    impl Default for TestVisitor {
        fn default() -> Self {
            TestVisitor {
                result: Ok(()),
            }
        }
    }

    let mut visitor = TestVisitor::default();
    let ast = ast::ClassSetItem::Empty(vec![]);
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

