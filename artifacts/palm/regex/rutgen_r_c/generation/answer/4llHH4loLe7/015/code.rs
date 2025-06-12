// Answer 0

#[test]
fn test_visit_post_empty() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let mut translator = TranslatorI::new(&trans, pattern);
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    
    assert_eq!(translator.visit_post(&ast), Ok(()));
    assert_eq!(translator.pop(), Some(HirFrame::Expr(Hir::empty())));
}

#[test]
fn test_visit_post_literal() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "a";
    let mut translator = TranslatorI::new(&trans, pattern);
    let ast = Ast::Literal(Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Char, c: 'a' });

    assert_eq!(translator.visit_post(&ast), Ok(()));
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

#[test]
fn test_visit_post_unicode_class() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: true,
    };
    let pattern = "UnicodeClass";
    let mut translator = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Unicode(ClassUnicode {
        span: Span { start: 0, end: 14 },
        negated: false,
        kind: ClassUnicodeKind::Name("L"),
    }));

    assert_eq!(translator.visit_post(&ast), Ok(()));
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

#[test]
fn test_visit_post_perl_class() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "PerlClass";
    let mut translator = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(ast::Class::Perl(ClassPerl {
        span: Span { start: 0, end: 9 },
        kind: ClassPerlKind::Word,
        negated: false,
    }));

    assert_eq!(translator.visit_post(&ast), Ok(()));
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
}

#[test]
fn test_visit_post_bracketed_class() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: b'a', end: b'z' }]);
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "BracketedClass";
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 15 },
        negated: false,
        kind: ClassSet::Union,
    }));

    translator.push(HirFrame::ClassBytes(class_bytes));

    assert_eq!(translator.visit_post(&ast), Ok(()));
    assert!(matches!(translator.pop(), Some(HirFrame::Expr(_))));
    
    let cls = translator.pop().unwrap().unwrap_class_bytes();
    assert!(cls.iter().next().is_some());
}

