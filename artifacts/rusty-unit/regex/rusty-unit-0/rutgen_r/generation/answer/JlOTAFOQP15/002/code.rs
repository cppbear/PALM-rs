// Answer 0

#[derive(Debug)]
struct TestVisitor {
    visited_count: usize,
}

impl TestVisitor {
    fn new() -> Self {
        TestVisitor { visited_count: 0 }
    }
}

impl Visitor for TestVisitor {
    type Output = usize;
    type Err = ();
    
    fn start(&mut self) { }

    fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
        self.visited_count += 1;
        Ok(())
    }

    fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
        Err(())
    }

    fn finish(mut self) -> Result<Self::Output, Self::Err> {
        Ok(self.visited_count)
    }

    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Debug)]
struct TestHir;

struct TestInduct<'a> {
    hir: &'a TestHir,
    called: bool,
}

impl<'a> TestInduct<'a> {
    fn child(&self) -> &'a TestHir {
        self.hir
    }
}

#[derive(Debug)]
struct TestStack {
    induct: Option<TestInduct<'static>>,
}

impl TestStack {
    fn clear(&mut self) {
        self.induct = None;
    }

    fn induct(&mut self, _: &TestHir) -> Option<TestInduct<'static>> {
        if self.induct.is_none() {
            self.induct = Some(TestInduct { hir: &TestHir, called: true });
        }
        self.induct.clone()
    }

    fn pop(&mut self, _: TestInduct<'static>) -> Option<TestInduct<'static>> {
        self.induct.take()
    }
}

#[test]
fn test_visit_function() {
    let mut stack = TestStack { induct: None };
    let mut visitor = TestVisitor::new();
    let hir = &TestHir;

    let result = visit(&mut stack, hir, visitor);

    assert_eq!(result.is_err(), true);
}

