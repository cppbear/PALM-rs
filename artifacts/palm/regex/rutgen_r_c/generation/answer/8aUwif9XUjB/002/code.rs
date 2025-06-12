// Answer 0

#[test]
fn test_visit_non_inductive_ast() {
    struct MockVisitor {
        pub output: Result<(), &'static str>,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            Self { output: Ok(()) }
        }
    }
    
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = &'static str;

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            self.output.clone()
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            self.output.clone()
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            self.output
        }
    }
    
    struct MockAst;

    let visitor = &mut MockVisitor::new();
    let ast = &Ast::Empty(Span::default());

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(ast, visitor);

    assert!(result.is_ok());
}

#[test]
fn test_visit_with_non_inductive_and_some_case() {
    struct NonInductiveVisitor {
        pub output: Result<(), &'static str>,
    }

    impl NonInductiveVisitor {
        fn new() -> Self {
            Self { output: Err("error") }
        }
    }

    impl Visitor for NonInductiveVisitor {
        type Output = ();
        type Err = &'static str;

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            self.output
        }
    }

    let visitor = &mut NonInductiveVisitor::new();
    let ast = &Ast::Empty(Span::default());

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(ast, visitor);

    assert!(result.is_err());
}

