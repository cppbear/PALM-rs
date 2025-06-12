// Answer 0

fn test_visit_post_empty() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    let result = translator_i.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(translator_i.pop().map(|f| f.unwrap_expr().kind()), Some(HirKind::Empty));
}

fn test_visit_post_flags() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&trans, pattern);
    let ast_flags = SetFlags {
        span: Span { start: 0, end: 10 },
        flags: Flags {
            case_insensitive: Some(false),
            multi_line: Some(false),
            dot_matches_new_line: Some(false),
            swap_greed: Some(false),
            unicode: Some(true),
        },
    };
    let ast = Ast::Flags(ast_flags);
    let result = translator_i.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_perl_class() {
    #[derive(Clone)]
    struct MockPerlClass {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let class_perl = ast::Class::Perl(MockPerlClass {
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    });

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(class_perl);
    let result = translator_i.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_unicode_class() {
    let class_unicode = ast::Class::Unicode(ast::ClassUnicode {
        span: Span { start: 0, end: 10 },
        negated: false,
        kind: ast::ClassUnicodeKind::Named("Alphabetic".to_string()),
    });

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(class_unicode);
    let result = translator_i.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_bracketed_class() {
    let class_bracketed = ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 10 },
        negated: false,
        kind: ast::ClassSet::Normal,
    });

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&trans, pattern);
    let ast = Ast::Class(class_bracketed);
    let result = translator_i.visit_post(&ast);
    assert!(result.is_ok());
}

