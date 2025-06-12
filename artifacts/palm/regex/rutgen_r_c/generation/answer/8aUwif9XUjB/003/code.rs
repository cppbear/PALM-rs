// Answer 0

fn test_visit_with_valid_ast() -> Result<(), Box<dyn std::error::Error>> {
    struct TestVisitor {
        output: Vec<String>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<String>;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            self.output.push(format!("Visited: {:?}", ast));
            Err("Post visit failed".into())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
    }

    let mut visitor = TestVisitor { output: Vec::new() };
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Literal("a".into()); // Assuming a string literal can be transformed to an Ast::Literal.
    let result = heap_visitor.visit(&ast, visitor)?;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["Visited: Literal(\"a\")"]); // Checking the expected output.
    Ok(())
}

fn test_visit_with_induction() -> Result<(), Box<dyn std::error::Error>> {
    struct InductionVisitor {
        depth: usize,
    }

    impl Visitor for InductionVisitor {
        type Output = usize;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.depth = 0;
        }

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            self.depth += 1;
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err("Post visit failed".into())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.depth)
        }
    }

    let mut visitor = InductionVisitor { depth: 0 };
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Repetition(ast::Repetition {
        min: 1,
        max: Some(2),
        child: Box::new(Ast::Literal("b".into())),
    });

    let result = heap_visitor.visit(&ast, visitor)?;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // Check that we at least visited once due to the repetition.
    Ok(())
}

fn test_visit_with_err_on_post_visit() -> Result<(), Box<dyn std::error::Error>> {
    struct ErrorVisitor;

    impl Visitor for ErrorVisitor {
        type Output = ();
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err("Post visit error".into()) // Triggering an error condition on post visit.
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = ErrorVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let ast = Ast::Group(ast::Group {
        nodes: vec![Ast::Literal("c".into())],
    });

    let result = heap_visitor.visit(&ast, visitor);
    assert!(result.is_err()); // Check if error is returned.
    Ok(())
}

fn main() {
    // Running the test functions, catching panics if necessary.
    let _ = test_visit_with_valid_ast();
    let _ = test_visit_with_induction();
    let _ = test_visit_with_err_on_post_visit();
}

