// Answer 0

#[test]
fn test_visit_with_empty_ast() {
    struct TestVisitor {
        output: Vec<String>,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<String>;
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_post".to_string());
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.output.push("visit_alternation_in".to_string());
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let ast = Ast::Empty(Span::default());

    let result = heap_visitor.visit(&ast, visitor).unwrap();

    assert_eq!(result, vec!["visit_pre", "visit_post"]);
}

#[test]
fn test_visit_with_literal_ast() {
    struct TestVisitor {
        output: Vec<String>,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<String>;
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_post".to_string());
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.output.push("visit_alternation_in".to_string());
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let ast = Ast::Literal(Literal { value: 'a' }); // Ensure proper struct for Literal is defined.

    let result = heap_visitor.visit(&ast, visitor).unwrap();

    assert_eq!(result, vec!["visit_pre", "visit_post"]);
}

#[test]
fn test_visit_with_concatenation() {
    struct TestVisitor {
        output: Vec<String>,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<String>;
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_pre".to_string());
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("visit_post".to_string());
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.output.push("visit_alternation_in".to_string());
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let ast = Ast::Concat(Concat {
        expressions: vec![
            Ast::Literal(Literal { value: 'a' }), 
            Ast::Literal(Literal { value: 'b' })
        ],
    });

    let result = heap_visitor.visit(&ast, visitor).unwrap();

    assert_eq!(result, vec!["visit_pre", "visit_pre", "visit_post", "visit_pre", "visit_post", "visit_post"]);
}

