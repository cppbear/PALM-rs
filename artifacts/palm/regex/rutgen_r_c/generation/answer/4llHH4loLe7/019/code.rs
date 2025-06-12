// Answer 0

fn test_ast_empty() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Empty(Span { start: 0, end: 0 });

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.pop().unwrap().unwrap_expr().kind(), &HirKind::Empty);
}

fn test_ast_unicode_class() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let x = ast::ClassUnicode {
        span: Span { start: 0, end: 1 },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    let ast = Ast::Class(ast::Class::Unicode(x));

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert!(visitor.pop().unwrap().unwrap_expr().kind().is_class());
}

fn test_ast_perl_class_unicode() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let x = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    let ast = Ast::Class(ast::Class::Perl(x));

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert!(visitor.pop().unwrap().unwrap_expr().kind().is_class());
}

fn test_ast_class_bracketed() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let class_item = ast::ClassSetItem::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Char,
        c: 'a',
    });
    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 2 },
        negated: false,
        kind: ast::ClassSet::Set(vec![class_item]),
    }));

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert!(visitor.pop().unwrap().unwrap_expr().kind().is_class());
}

