// Answer 0

fn test_visit_post_empty() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    
    translator.visit_post(&ast).unwrap();
    
    assert_eq!(translator.stack.borrow().len(), 1);
    assert!(matches!(translator.stack.borrow().last().unwrap(),
                    HirFrame::Expr(hir) if hir.kind() == &HirKind::Empty));
}

fn test_visit_post_unicode_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    
    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 0 },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    
    let ast = Ast::Class(ast::Class::Unicode(unicode_class));
    
    translator.visit_post(&ast).unwrap();
    
    assert_eq!(translator.stack.borrow().len(), 1);
    assert!(matches!(translator.stack.borrow().last().unwrap(), 
                    HirFrame::Expr(_)));
}

fn test_visit_post_empty_unicode_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    
    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 0 },
        negated: false,
        kind: ast::ClassUnicodeKind::Named("NotAValidProperty".to_string()),
    };
    
    let ast = Ast::Class(ast::Class::Unicode(unicode_class));
    
    let result = translator.visit_post(&ast);
    
    assert!(result.is_err());
}

fn test_visit_post_empty_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    
    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 0 },
        negated: false,
        kind: ast::ClassUnicodeKind::Named("ValidProperty".to_string()),
    };
    
    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassSet {
        span: Span { start: 0, end: 0 },
        negated: false,
        items: vec![],
    }));
    
    let result = translator.visit_post(&ast);
    
    assert!(result.is_err());
}

fn test_visit_post_perl_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    
    let perl_class = ast::ClassPerl {
        span: Span { start: 0, end: 0 },
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let ast = Ast::Class(ast::Class::Perl(perl_class));
    
    translator.visit_post(&ast).unwrap();
    
    assert_eq!(translator.stack.borrow().len(), 1);
    assert!(matches!(translator.stack.borrow().last().unwrap(),
                    HirFrame::Expr(_)));
}

fn test_visit_post_perl_unicode_class_invalid() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    
    let perl_class = ast::ClassPerl {
        span: Span { start: 0, end: 0 },
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    
    let ast = Ast::Class(ast::Class::Perl(perl_class));
    
    let result = translator.visit_post(&ast);
    
    assert!(result.is_ok());
    assert_eq!(translator.stack.borrow().len(), 1);
    assert!(matches!(translator.stack.borrow().last().unwrap(),
                    HirFrame::Expr(_)));
}

