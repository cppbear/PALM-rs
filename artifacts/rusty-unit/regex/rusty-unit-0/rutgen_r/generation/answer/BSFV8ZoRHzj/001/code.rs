// Answer 0

#[test]
fn test_visit_class_set_item_pre_non_bracketed() {
    struct TestVisitor;
    impl TestVisitor {
        fn fmt_class_bracketed_pre(&mut self, _x: &str) -> Result<(), ()> {
            // This function should never be called in this test
            panic!("fmt_class_bracketed_pre should not be called");
        }
    }
    
    impl TestVisitor {
        type Err = ();
        
        fn visit_class_set_item_pre(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<(), Self::Err> {
            match *ast {
                ast::ClassSetItem::Bracketed(ref x) => {
                    self.fmt_class_bracketed_pre(x)
                }
                _ => Ok(()),
            }
        }
    }

    enum ClassSetItem {
        Bracketed(String),
        OtherItem,
    }
    
    let mut visitor = TestVisitor;
    let item = ClassSetItem::OtherItem;
    let result = visitor.visit_class_set_item_pre(&item);
    assert_eq!(result, Ok(()));
}

