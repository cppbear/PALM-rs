// Answer 0

fn test_visit_pre_err() {
    struct MockVisitor {
        should_error: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            if self.should_error {
                Err(())
            } else {
                Ok(())
            }
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor { should_error: true };
    let ast = Ast::Empty(Span::default()); // Use a default span

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&ast, visitor);
    assert!(result.is_err());
}

fn test_visit_induct_err() {
    struct MockVisitor {
        induce_error: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor { induce_error: true };
    let ast = Ast::Empty(Span::default());

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&ast, visitor);
    assert!(result.is_ok());
}

fn test_visit_post_no_induct() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Empty(Span::default());
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&ast, visitor);
    assert!(result.is_ok());
}

