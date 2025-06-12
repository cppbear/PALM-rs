// Answer 0

#[test]
fn test_visit_class_with_inductive_case() {
    struct TestVisitor {
        visited: bool,
    }

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &BinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    struct TestVisitorStack {
        stack_class: Vec<(ClassInduct, ClassFrame)>,
        visited: bool,
    }

    impl TestVisitorStack {
        fn new() -> Self {
            TestVisitorStack {
                stack_class: Vec::new(),
                visited: false,
            }
        }

        fn visit_class_pre(&mut self, _ast: &ClassInduct, _visitor: &mut TestVisitor) -> Result<(), ()> {
            self.visited = true;
            Ok(())
        }

        fn visit_class_post(&mut self, _ast: &ClassInduct, _visitor: &mut TestVisitor) -> Result<(), ()> {
            Err(())
        }

        fn induct_class(&self, _ast: &ClassInduct) -> Option<InductiveClassFrame> {
            Some(InductiveClassFrame::new())
        }

        fn pop_class(&mut self, _frame: ClassFrame) -> Option<ClassFrame> {
            None // Mimicking end of stack
        }
    }

    struct InductiveClassFrame;

    impl InductiveClassFrame {
        fn new() -> Self {
            InductiveClassFrame {}
        }

        fn child(&self) -> ClassInduct {
            ClassInduct {}
        }
    }

    struct ClassInduct;

    impl ClassInduct {
        fn from_bracketed(_ast: &ast::ClassBracketed) -> Self {
            ClassInduct {}
        }
    }

    struct ClassFrame;

    let mut visitor_stack = TestVisitorStack::new();
    let visitor = &mut TestVisitor { visited: false };
    let ast = ClassInduct::from_bracketed(&ast::ClassBracketed {});
    
    let result = visitor_stack.visit_class(&ast, visitor);
    assert_eq!(result.is_err(), true);
    assert!(visitor_stack.visited);
}

#[test]
#[should_panic]
fn test_visit_class_with_non_inductive_case() {
    struct FailingVisitor;

    impl Visitor for FailingVisitor {
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &BinaryOp) -> Result<(), Self::Err> {
            panic!("This should not be called");
        }
    }

    struct FailingVisitorStack {
        stack_class: Vec<(ClassInduct, ClassFrame)>,
    }

    impl FailingVisitorStack {
        fn new() -> Self {
            FailingVisitorStack {
                stack_class: Vec::new(),
            }
        }

        fn visit_class_pre(&mut self, _ast: &ClassInduct, _visitor: &mut FailingVisitor) -> Result<(), ()> {
            Ok(())
        }

        fn visit_class_post(&mut self, _ast: &ClassInduct, _visitor: &mut FailingVisitor) -> Result<(), ()> {
            panic!("This should not succeed");
        }

        fn induct_class(&self, _ast: &ClassInduct) -> Option<InductiveClassFrame> {
            None
        }

        fn pop_class(&mut self, _frame: ClassFrame) -> Option<ClassFrame> {
            None
        }
    }

    let mut visitor_stack = FailingVisitorStack::new();
    let visitor = &mut FailingVisitor {};
    let ast = ClassInduct::from_bracketed(&ast::ClassBracketed {});

    let _ = visitor_stack.visit_class(&ast, visitor);
}

