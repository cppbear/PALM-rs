// Answer 0

#[test]
fn test_visit_pre_class_unicode() {
    struct TestTranslator {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Translator for TestTranslator {
        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let pattern = "[a-z]";
    let translator = TestTranslator {
        flags: Flags { unicode: Some(true), ..Flags::default() },
        stack: RefCell::new(vec![]),
    };

    let ast = Ast::Class(ast::Class::Bracketed(vec![]));
    let mut visitor = TranslatorI { trans: &translator, pattern };

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
    assert!(matches!(translator.stack.borrow()[0], HirFrame::ClassUnicode(_)));
}

#[test]
fn test_visit_pre_class_bytes() {
    struct TestTranslator {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Translator for TestTranslator {
        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let pattern = "[0-9]";
    let translator = TestTranslator {
        flags: Flags { unicode: Some(false), ..Flags::default() },
        stack: RefCell::new(vec![]),
    };

    let ast = Ast::Class(ast::Class::Bracketed(vec![]));
    let mut visitor = TranslatorI { trans: &translator, pattern };

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
    assert!(matches!(translator.stack.borrow()[0], HirFrame::ClassBytes(_)));
}

#[test]
fn test_visit_pre_group() {
    struct TestTranslator {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Translator for TestTranslator {
        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let pattern = "(abc)";
    let translator = TestTranslator {
        flags: Flags::default(),
        stack: RefCell::new(vec![]),
    };

    let ast = Ast::Group(Group { span: Span::default(), kind: GroupKind::Capturing, ast: Box::new(Ast::Empty(Span::default())) });
    let mut visitor = TranslatorI { trans: &translator, pattern };

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
    assert!(matches!(translator.stack.borrow()[0], HirFrame::Group { .. }));
}

#[test]
fn test_visit_pre_concat() {
    struct TestTranslator {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Translator for TestTranslator {
        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let pattern = "abc";
    let translator = TestTranslator {
        flags: Flags::default(),
        stack: RefCell::new(vec![]),
    };

    let ast = Ast::Concat(Concat { span: Span::default(), asts: vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))] });
    let mut visitor = TranslatorI { trans: &translator, pattern };

    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
    assert!(matches!(translator.stack.borrow()[0], HirFrame::Concat));
}

