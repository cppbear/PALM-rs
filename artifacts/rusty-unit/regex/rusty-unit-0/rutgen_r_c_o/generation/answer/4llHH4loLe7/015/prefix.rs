// Answer 0

#[test]
fn test_visit_post_literal() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };
    
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::Character,
        c: 'a',
    };
    
    let ast = Ast::Literal(literal);
    
    translator.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_unicode_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };
    
    let span = Span { start: 0, end: 1 };

    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class = Class::Unicode(unicode_class);
    let ast = Ast::Class(class);

    translator.push(HirFrame::ClassBytes(ClassBytes::new(vec![
        ClassBytesRange { start: 97, end: 122 },
    ])));

    translator.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_bracketed_class() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };

    let span = Span { start: 0, end: 10 };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]);
    let bracketed_class = ast::Class::Bracketed(ast::ClassBracketed {
        span,
        negated: false,
        kind: ast::ClassSet::Union,
    });
    
    let ast = Ast::Class(bracketed_class);
    
    translator.push(HirFrame::ClassBytes(class_bytes));
    translator.visit_post(&ast).unwrap();
}

