// Answer 0

#[test]
fn test_visit_post_alternation_with_pop() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { frames: Vec::new() }
        }

        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }
    }

    let mut visitor = TestVisitor::new();

    // Prepare a valid expression to push onto the stack
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));

    let ast = Ast::Alternation(vec![]); // Placeholder for an empty Alternation
    let result = visitor.visit_post(&ast);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_post_empty_pop_on_alternation() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { frames: Vec::new() }
        }

        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }
    }

    let mut visitor = TestVisitor::new();

    // No frames are pushed, this should panic
    let ast = Ast::Alternation(vec![]); // Placeholder for an empty Alternation
    let _result = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_alternation_with_non_empty_pop() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { frames: Vec::new() }
        }

        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }
    }

    let mut visitor = TestVisitor::new();

    // Prepare a valid expression to push onto the stack
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));

    let ast = Ast::Alternation(vec![]); // Placeholder for an empty Alternation
    let result = visitor.visit_post(&ast);

    assert!(result.is_ok());
}

