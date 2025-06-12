// Answer 0

#[test]
fn test_visit_post_empty_class_not_allowed() {
    struct MockVisitor {
        stack: Vec<HirFrame>,
        unicode_flag: bool,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            Self { stack: Vec::new(), unicode_flag: true }
        }

        fn push(&mut self, frame: HirFrame) {
            self.stack.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.stack.pop()
        }

        fn flags(&self) -> Flags {
            Flags { unicode: self.unicode_flag }
        }

        fn error(&self, _span: Span, kind: ErrorKind) -> Result<()> {
            Err(kind.into())
        }
        
        fn unicode_fold_and_negate(&self, _negated: bool, _cls: &mut Class) {}
    }

    let mut visitor = MockVisitor::new();
    let ast = Ast::Class(Class::Bracketed(Box::new(ClassAst { negated: true, span: Span::default() })));

    visitor.push(HirFrame::Expr(Hir::class(Class::Unicode(vec![]))));

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::EmptyClassNotAllowed);
}

#[test]
fn test_visit_post_literal() {
    struct MockVisitor {
        stack: Vec<HirFrame>,
        unicode_flag: bool,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            Self { stack: Vec::new(), unicode_flag: true }
        }

        fn push(&mut self, frame: HirFrame) {
            self.stack.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.stack.pop()
        }

        fn flags(&self) -> Flags {
            Flags { unicode: self.unicode_flag }
        }
        
        fn hir_literal(&self, _x: &Literal) -> Result<Hir> {
            Ok(Hir::literal("test".to_string()))
        }
        
        fn error(&self, _span: Span, _kind: ErrorKind) -> Result<()> {
            Ok(())
        }
    }
    
    let mut visitor = MockVisitor::new();
    let ast = Ast::Literal(Literal::new("test".into()));

    visitor.visit_post(&ast).unwrap();

    assert_eq!(visitor.stack.len(), 1);
}

#[test]
fn test_visit_post_unicode_class() {
    struct MockVisitor {
        stack: Vec<HirFrame>,
        unicode_flag: bool,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            Self { stack: Vec::new(), unicode_flag: true }
        }

        fn push(&mut self, frame: HirFrame) {
            self.stack.push(frame);
        }

        fn pop(&mut self) -> Option<HirFrame> {
            self.stack.pop()
        }

        fn flags(&self) -> Flags {
            Flags { unicode: self.unicode_flag }
        }

        fn hir_unicode_class(&self, _x: &UnicodeClass) -> Result<Class> {
            Ok(Class::Unicode(vec![]))
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Result<()> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let ast = Ast::Class(Class::Unicode(UnicodeClass::default()));

    visitor.visit_post(&ast).unwrap();
    
    assert_eq!(visitor.stack.len(), 1);
}

