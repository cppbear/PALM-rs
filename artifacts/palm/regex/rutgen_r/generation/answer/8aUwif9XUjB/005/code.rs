// Answer 0

fn visit_test() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        visited_pre: bool,
        visited_post: bool,
        finished: bool,
        output: Option<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                visited_pre: false,
                visited_post: false,
                finished: false,
                output: None,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = String;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.visited_pre = false;
            self.visited_post = false;
        }

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            if self.finished { return Err("Visitor already finished".into()); }
            self.visited_pre = true;
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            if !self.visited_pre { return Err("Visit pre not called".into()); }
            self.visited_post = true;
            self.output = Some("Visited".into());
            Ok(())
        }

        fn finish(&mut self) -> Result<Self::Output, Self::Err> {
            self.finished = true;
            Ok(self.output.clone().unwrap_or_else(|| "Nothing visited".into()))
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    struct MockAst {
        child: Option<Box<MockAst>>,
    }

    impl MockAst {
        fn new(child: Option<Box<MockAst>>) -> Self {
            Self { child }
        }

        fn child(&self) -> &Option<Box<MockAst>> {
            &self.child
        }
    }

    struct MockInductor;

    impl MockInductor {
        fn induct(&self, ast: &MockAst, visitor: &mut MockVisitor) -> Result<Option<Box<MockAst>>, Box<dyn std::error::Error>> {
            visitor.visit_pre(ast)?;
            Ok(ast.child.clone())
        }
    }

    let mut visitor = MockVisitor::new();
    let ast = MockAst::new(Some(Box::new(MockAst::new(None))));
    let mut inductor = MockInductor;
    
    let result = inductor.induct(&ast, &mut visitor)?;
    if let Some(child) = result {
        visitor.visit_post(&ast)?;
        assert_eq!(visitor.output, Some("Visited".into()));
    }

    Ok(())
}

#[test]
fn test_visit() {
    let _ = visit_test().unwrap();
}

#[should_panic]
fn test_visit_should_panic() {
    visit_test().unwrap_err();
}

