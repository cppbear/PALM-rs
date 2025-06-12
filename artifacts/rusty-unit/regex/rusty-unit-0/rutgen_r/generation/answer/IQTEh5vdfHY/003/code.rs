// Answer 0

#[derive(Debug)]
struct TestVisitor {
    should_panic: bool,
}

impl Visitor for TestVisitor {
    type Err = String;

    fn visit_class_set_item_post(&mut self, _item: ClassSetItem) -> Result<(), Self::Err> {
        if self.should_panic {
            Err("Visitor panic".into())
        } else {
            Ok(())
        }
    }

    fn visit_class_set_binary_op_post(&mut self, _op: ClassBinaryOp) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[test]
fn test_visit_class_post_item_panic() {
    let ast = ClassInduct::Item(ClassSetItem {});
    let mut visitor = TestVisitor { should_panic: true };

    let result = visit_class_post(&(), &ast, &mut visitor);
    assert_eq!(result, Err("Visitor panic".into()));
}

#[test]
fn test_visit_class_post_item_no_panic() {
    let ast = ClassInduct::Item(ClassSetItem {});
    let mut visitor = TestVisitor { should_panic: false };

    let result = visit_class_post(&(), &ast, &mut visitor);
    assert_eq!(result, Ok(()));
}

