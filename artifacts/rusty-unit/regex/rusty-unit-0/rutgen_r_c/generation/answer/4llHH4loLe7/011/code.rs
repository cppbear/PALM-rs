// Answer 0

fn test_visit_post_empty_class_not_allowed() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(Vec::new()),
                flags: Cell::new(Flags {
                    unicode: Some(true),
                    ..Flags::default()
                }),
                allow_invalid_utf8: false,
            }
        }
    }

    struct MockVisitor<'a> {
        trans: &'a MockTranslator,
        pattern: &'a str,
    }

    impl<'a> TranslatorI<'a, 'a> {
        fn new(trans: &'a MockTranslator, pattern: &'a str) -> Self {
            TranslatorI { trans, pattern }
        }
    }

    let trans = MockTranslator::new();
    let pattern = "[a]";
    let mut visitor = TranslatorI::new(&trans, pattern);
    
    // Create a ClassBracketed with negated set and empty class
    let ast_class = ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 3 },
        negated: true,
        kind: ast::ClassSet::Normal,
    });
    let ast = ast::Ast::Class(ast_class);

    let result = visitor.visit_post(&ast);

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ErrorKind::EmptyClassNotAllowed);
    }
}

fn test_visit_post_literal() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(Vec::new()),
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
                allow_invalid_utf8: false,
            }
        }
    }

    struct MockVisitor<'a> {
        trans: &'a MockTranslator,
        pattern: &'a str,
    }

    impl<'a> TranslatorI<'a, 'a> {
        fn new(trans: &'a MockTranslator, pattern: &'a str) -> Self {
            TranslatorI { trans, pattern }
        }
    }

    let trans = MockTranslator::new();
    let pattern = "a";
    let mut visitor = TranslatorI::new(&trans, pattern);

    let ast_literal = ast::Ast::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Character,
        c: 'a',
    });

    let result = visitor.visit_post(&ast_literal);

    assert!(result.is_ok());
}

fn test_visit_post_perl_class() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(Vec::new()),
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
                allow_invalid_utf8: false,
            }
        }
    }

    struct MockVisitor<'a> {
        trans: &'a MockTranslator,
        pattern: &'a str,
    }

    impl<'a> TranslatorI<'a, 'a> {
        fn new(trans: &'a MockTranslator, pattern: &'a str) -> Self {
            TranslatorI { trans, pattern }
        }
    }

    let trans = MockTranslator::new();
    let pattern = "\\d";
    let mut visitor = TranslatorI::new(&trans, pattern);

    let ast_class = ast::Class::Perl(ast::ClassPerl {
        span: Span { start: 0, end: 2 },
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    });
    let ast = ast::Ast::Class(ast_class);

    let _ = visitor.visit_post(&ast);
}

