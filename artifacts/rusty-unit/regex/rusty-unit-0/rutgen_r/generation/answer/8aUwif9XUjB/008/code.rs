// Answer 0

fn visit_test() -> Result<(), Box<dyn std::error::Error>> {
    struct TestVisitor {
        output: Vec<String>,
        err: Option<String>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { output: Vec::new(), err: None }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<String>;
        type Err = String;

        fn start(&mut self) {
            self.output.push("Start".to_string());
        }

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("Visit Pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.output.push("Visit Post".to_string());
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.output.push("Visit Alternation In".to_string());
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            if let Some(err) = self.err {
                Err(err)
            } else {
                Ok(self.output)
            }
        }
    }

    struct TestAst;

    struct TestFrame;

    impl TestFrame {
        fn child(&self) -> &TestAst {
            &TestAst
        }
    }

    struct TestStack {
        stack: Vec<(TestAst, TestFrame)>,
    }

    impl TestStack {
        fn new() -> Self {
            Self { stack: Vec::new() }
        }

        fn push(&mut self, item: (TestAst, TestFrame)) {
            self.stack.push(item);
        }

        fn pop(&mut self) -> Option<(TestAst, TestFrame)> {
            self.stack.pop()
        }

        fn induct(&self, _ast: &TestAst, _visitor: &mut TestVisitor) -> Result<Option<TestFrame>, String> {
            Ok(Some(TestFrame))
        }
    }

    let mut stack = TestStack::new();
    let ast = TestAst;
    let mut visitor = TestVisitor::new();

    let result = stack.induct(&ast, &mut visitor)?;
    if let Some(frame) = result {
        stack.push((ast, frame));
    }

    while let Some((post_ast, frame)) = stack.pop() {
        visitor.visit_post(&post_ast)?;
        if let Some(x) = stack.pop() {
            visitor.visit_alternation_in()?;
            stack.push((post_ast, x));
            break;
        }
    }

    Ok(())
}

#[test]
fn test_visit() {
    let result = visit_test();
    assert!(result.is_ok());
}

