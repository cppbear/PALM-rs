// Answer 0

fn test_visit_post_empty() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    assert!(translator.visit_post(&ast).is_ok());
    assert_eq!(translator.pop().unwrap().unwrap_expr().kind(), &HirKind::Empty);
}

fn test_visit_post_flags() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Flags(SetFlags { span: Span { start: 0, end: 0 }, flags: Flags::default() });
    assert!(translator.visit_post(&ast).is_ok());
    assert!(translator.flags().case_insensitive.is_none());
}

fn test_visit_post_literal() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Literal(Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Character, c: 'a' });
    assert!(translator.visit_post(&ast).is_ok());
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

fn test_visit_post_dot() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Dot(Span { start: 0, end: 1 });
    assert!(translator.visit_post(&ast).is_ok());
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

fn test_visit_post_assertion() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Assertion(Assertion { span: Span { start: 0, end: 1 }, kind: AssertionKind::StartLine });
    assert!(translator.visit_post(&ast).is_ok());
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

fn test_visit_post_class_unicode() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Class(ast::Class::Unicode(ClassUnicode { span: Span { start: 0, end: 1 }, negated: false, kind: ClassUnicodeKind::OneLetter('a') }));
    assert!(translator.visit_post(&ast).is_ok());
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

fn test_visit_post_concat() {
    let mut translator = {
        let trans = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            allow_invalid_utf8: false,
        };
        TranslatorI::new(&trans, "")
    };
    let ast = Ast::Concat(vec![Ast::Literal(Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Character, c: 'a' })]);
    assert!(translator.visit_post(&ast).is_ok());
    
    let expr = translator.pop().unwrap().unwrap_expr();
    assert!(expr.kind().is_empty()); // Testing the expr to ensure it has kind empty after visit_post on concat
}

