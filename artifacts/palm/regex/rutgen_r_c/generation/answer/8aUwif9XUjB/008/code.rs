// Answer 0

fn test_visit_success() {
    struct MockVisitor {
        pre_invocations: usize,
        post_invocations: usize,
        alternation_in_invocations: usize,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                pre_invocations: 0,
                post_invocations: 0,
                alternation_in_invocations: 0,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            self.pre_invocations += 1; 
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            self.post_invocations += 1;
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.alternation_in_invocations += 1;
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    
    let ast = Ast::Alternation(Box::new(Ast::Empty(Span::dummy())));
    
    let result = heap_visitor.visit(&ast, visitor);
    
    assert!(result.is_ok());
    assert_eq!(visitor.pre_invocations, 2); // One for the pre visit of Alternation and one for its child
    assert_eq!(visitor.post_invocations, 2); // One for post visit of its child and one for Alternation
    assert_eq!(visitor.alternation_in_invocations, 1);
}

fn test_visit_err() {
    struct ErrVisitor {
        invoked: bool,
    }

    impl ErrVisitor {
        fn new() -> Self {
            ErrVisitor { invoked: false }
        }
    }

    impl Visitor for ErrVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            // Simulating failure on post visit
            Err(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.invoked = true;
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = ErrVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Alternation(Box::new(Ast::Empty(Span::dummy())));
    
    let result = heap_visitor.visit(&ast, visitor);
    
    assert!(result.is_err());
    assert!(!visitor.invoked); // Ensure alternation_in was not invoked
}

