// Answer 0

#[test]
fn test_hir_unicode_class_one_letter_property_not_found() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let ast_class = ast::ClassUnicode {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    
    translator.hir_unicode_class(&ast_class);
}

#[test]
fn test_hir_unicode_class_one_letter_property_value_not_found() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let ast_class = ast::ClassUnicode {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('b'),
    };
    
    translator.hir_unicode_class(&ast_class);
}

#[test]
fn test_hir_unicode_class_one_letter_success() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let ast_class = ast::ClassUnicode {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('c'),
    };
    
    translator.hir_unicode_class(&ast_class);
}

