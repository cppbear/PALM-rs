// Answer 0

#[test]
fn test_visit_post_empty() {
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

#[test]
fn test_visit_post_literal() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "a";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Unicode,
        c: 'a',
    });

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_ok());
    assert!(matches!(visitor.pop().unwrap(), HirFrame::Expr(Hir { kind: HirKind::Literal(_), .. })));
}

#[test]
fn test_visit_post_class_unicode() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "\\p{L}";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Unicode(ClassUnicode {
        span: Span { start: 0, end: 4 },
        negated: false,
        kind: ClassUnicodeKind::OneLetter('L'),
    }));

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_ok());
    assert!(matches!(visitor.pop().unwrap(), HirFrame::Expr(Hir { kind: HirKind::Class(_), .. })));
}

#[test]
fn test_visit_post_class_perl() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "\\d";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Perl(ClassPerl {
        span: Span { start: 0, end: 2 },
        kind: ClassPerlKind::Digit,
        negated: false,
    }));

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_ok());
    assert!(matches!(visitor.pop().unwrap(), HirFrame::Expr(Hir { kind: HirKind::Class(_), .. })));
}

#[test]
fn test_visit_post_bracketed_class() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "[abc]";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 5 },
        negated: false,
        kind: ClassSet::Normal,
    }));

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_ok());
    assert!(matches!(visitor.pop().unwrap(), HirFrame::Expr(Hir { kind: HirKind::Class(_), .. })));
}

#[test]
fn test_visit_post_bracketed_class_empty() {
    let trans = Translator {
        stack: RefCell::new(vec![HirFrame::ClassUnicode(ClassUnicode::empty())]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "[abc]";
    let mut visitor = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 5 },
        negated: false,
        kind: ClassSet::Normal,
    }));

    let result = visitor.visit_post(&ast);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: pattern.to_string(),
        span: ast.span(),
    });
}

