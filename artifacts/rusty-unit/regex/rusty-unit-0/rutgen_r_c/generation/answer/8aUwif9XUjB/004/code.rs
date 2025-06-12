// Answer 0

fn test_visit_with_valid_ast() {
    struct MockVisitor {
        pre_visit_called: bool,
        post_visit_called: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                pre_visit_called: false,
                post_visit_called: false,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.pre_visit_called = true;
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.post_visit_called = true;
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Concat(Vec::new());

    let result = heap_visitor.visit(&ast, visitor);

    assert!(result.is_ok());
    assert!(visitor.pre_visit_called);
    assert!(visitor.post_visit_called);
}

fn test_visit_with_induct_fail() {
    struct MockVisitor {
        pre_visit_called: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                pre_visit_called: false,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.pre_visit_called = true;
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Err(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Group(Vec::new());

    let result = heap_visitor.visit(&ast, visitor);

    assert!(result.is_err());
    assert!(visitor.pre_visit_called);
}

fn test_visit_with_empty_stack() {
    struct MockVisitor {
        pre_visit_called: bool,
        post_visit_called: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                pre_visit_called: false,
                post_visit_called: false,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.pre_visit_called = true;
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.post_visit_called = true;
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Empty(Vec::new());

    let result = heap_visitor.visit(&ast, visitor);

    assert!(result.is_ok());
    assert!(visitor.pre_visit_called);
    assert!(visitor.post_visit_called);
}

