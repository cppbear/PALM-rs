// Answer 0

#[test]
fn test_visit_post_empty_ast() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }

        fn hir_literal(&self, _: &str) -> Result<Hir> {
            Ok(Hir::literal("test"))
        }

        fn flags(&self) -> Flags {
            Flags::default()
        }
    }

    let mut visitor = TestVisitor { frames: vec![] };
    let ast = Ast::Empty(None);
    visitor.visit_post(&ast).unwrap();
    assert_eq!(visitor.frames.len(), 1);
}

#[test]
fn test_visit_post_literal_ast() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }

        fn hir_literal(&self, lit: &str) -> Result<Hir> {
            Ok(Hir::literal(lit))
        }
    }

    let mut visitor = TestVisitor { frames: vec![] };
    let ast = Ast::Literal("example".to_string());
    visitor.visit_post(&ast).unwrap();
    assert_eq!(visitor.frames.len(), 1);
}

#[test]
fn test_visit_post_dot_ast() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }

        fn hir_dot(&self, _: Span) -> Result<Hir> {
            Ok(Hir::dot())
        }
    }

    let mut visitor = TestVisitor { frames: vec![] };
    let ast = Ast::Dot(Span::default());
    visitor.visit_post(&ast).unwrap();
    assert_eq!(visitor.frames.len(), 1);
}

#[test]
fn test_visit_post_repetition_ast() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }

        fn hir_repetition(&self, _: &Repetition, _: Hir) -> Hir {
            Hir::repetition()
        }
    }

    let mut visitor = TestVisitor { frames: vec![] };
    visitor.push(HirFrame::Expr(Hir::literal("test")));
    let ast = Ast::Repetition(Repetition { min: 1, max: None });
    visitor.visit_post(&ast).unwrap();
    assert_eq!(visitor.frames.len(), 1);
}

#[test]
fn test_visit_post_concat_ast() {
    struct TestVisitor {
        frames: Vec<HirFrame>,
    }

    impl TestVisitor {
        fn push(&mut self, frame: HirFrame) {
            self.frames.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.frames.pop()
        }
    }

    let mut visitor = TestVisitor { frames: vec![] };
    visitor.push(HirFrame::Expr(Hir::literal("first")));
    visitor.push(HirFrame::Expr(Hir::literal("second")));
    let ast = Ast::Concat(vec![]);
    visitor.visit_post(&ast).unwrap();
    assert_eq!(visitor.frames.len(), 1); // Expecting to have one concatenated expression.
}

