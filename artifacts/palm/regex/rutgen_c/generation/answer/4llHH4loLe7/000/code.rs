// Answer 0

#[test]
fn test_visit_post_empty_ast() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    translator_i.visit_post(&ast).unwrap();
    
    let result = translator_i.pop();
    match result {
        Some(HirFrame::Expr(hir)) => {
            assert!(hir.kind().is_empty());
        }
        _ => panic!("Expected HirFrame::Expr with empty Hir"),
    }
}

#[test]
fn test_visit_post_literal_ast() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "a";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    
    let ast = Ast::Literal(Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Unicode, c: 'a' });
    translator_i.visit_post(&ast).unwrap();
    
    let result = translator_i.pop();
    match result {
        Some(HirFrame::Expr(hir)) => {
            if let HirKind::Literal(literal) = hir.kind() {
                assert_eq!(literal.c, 'a');
            } else {
                panic!("Expected Literal kind");
            }
        }
        _ => panic!("Expected HirFrame::Expr with Literal Hir"),
    }
}

#[test]
fn test_visit_post_dot_ast() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            dot_matches_new_line: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = ".";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    
    let ast = Ast::Dot(Span { start: 0, end: 1 });
    translator_i.visit_post(&ast).unwrap();
    
    let result = translator_i.pop();
    match result {
        Some(HirFrame::Expr(hir)) => {
            assert!(matches!(hir.kind(), HirKind::Dot(false)));
        }
        _ => panic!("Expected HirFrame::Expr with Dot Hir"),
    }
}

#[test]
fn test_visit_post_assertion_ast() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "^";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    
    let ast = Ast::Assertion(Assertion { span: Span { start: 0, end: 1 }, kind: AssertionKind::StartText });
    translator_i.visit_post(&ast).unwrap();
    
    let result = translator_i.pop();
    match result {
        Some(HirFrame::Expr(hir)) => {
            assert!(matches!(hir.kind(), HirKind::Anchor(hir::Anchor::StartText)));
        }
        _ => panic!("Expected HirFrame::Expr with Anchor Hir"),
    }
}

#[test]
fn test_visit_post_class_unicode_ast() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "[\\p{L}]";
    let mut translator_i = TranslatorI::new(&translator, pattern);
    
    let ast = Ast::Class(ast::Class::Unicode(ClassUnicode {
        span: Span { start: 0, end: 7 },
        negated: false,
        kind: ClassUnicodeKind::OneLetter('L'),
    }));
    translator_i.visit_post(&ast).unwrap();
    
    let result = translator_i.pop();
    match result {
        Some(HirFrame::Expr(hir)) => {
            if let HirKind::Class(class) = hir.kind() {
                if let Class::Unicode(_) = class {
                    // Test passed
                } else {
                    panic!("Expected Unicode class");
                }
            } else {
                panic!("Expected Class kind");
            }
        }
        _ => panic!("Expected HirFrame::Expr with Class Hir"),
    }
}

