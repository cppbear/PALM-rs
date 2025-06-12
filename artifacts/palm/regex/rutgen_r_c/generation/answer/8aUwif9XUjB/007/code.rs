// Answer 0

fn test_visit_success() {
    struct DummyVisitor {
        finished: bool,
        pre_called: bool,
        post_called: bool,
    }

    impl DummyVisitor {
        fn new() -> Self {
            Self {
                finished: false,
                pre_called: false,
                post_called: false,
            }
        }
    }

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.pre_called = true;
            Ok(())
        }
        fn visit_post<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.post_called = true;
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Err(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            self.finished = true;
            Ok(())
        }
    }

    let mut visitor = DummyVisitor::new();
    let ast = Ast::Alternation(ast::Alternation { /* initialize fields */ });
    let mut heap_visitor = HeapVisitor::new();
    
    let result = heap_visitor.visit(&ast, visitor);
    
    assert!(result.is_ok());
    assert!(visitor.finished);
    assert!(visitor.pre_called);
    assert!(visitor.post_called);
}

fn test_visit_induction_failure() {
    struct FailingVisitor {
        pre_called: bool,
        post_called: bool,
    }

    impl FailingVisitor {
        fn new() -> Self {
            Self {
                pre_called: false,
                post_called: false,
            }
        }
    }

    impl Visitor for FailingVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.pre_called = true;
            Ok(())
        }
        fn visit_post<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.post_called = true;
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Alternation(ast::Alternation { /* initialize fields */ });
    let mut heap_visitor = HeapVisitor::new();
    let failing_visitor = FailingVisitor::new();

    let result = heap_visitor.visit(&ast, failing_visitor); 
    
    assert!(result.is_ok());
}

fn test_visit_stack_pop() {
    struct StackPopVisitor {
        pre_called: bool,
        post_called: bool,
    }

    impl StackPopVisitor {
        fn new() -> Self {
            Self {
                pre_called: false,
                post_called: false,
            }
        }
    }

    impl Visitor for StackPopVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.pre_called = true;
            Ok(())
        }
        fn visit_post<'a>(&mut self, _ast: &'a Ast) -> Result<(), Self::Err> {
            self.post_called = true;
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Alternation(ast::Alternation { /* initialize fields */ });
    let mut heap_visitor = HeapVisitor::new();
    let visitor = StackPopVisitor::new();

    let result = heap_visitor.visit(&ast, visitor); 
    
    assert!(result.is_ok());
}

