// Answer 0

#[test]
fn test_visit_class_set_item_post_literal_byte() {
    use std::cell::RefCell;
    use std::collections::HashMap;

    let mut stack: Vec<HirFrame> = Vec::new();
    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };
    let trans = Translator {
        stack: RefCell::new(stack),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let pattern = "a";
    let mut translator_instance = TranslatorI::new(&trans, pattern);

    let x = ast::Literal {
        span: Span { start: Position::new(0), end: Position::new(1) },
        kind: ast::LiteralKind::Character,
        c: 'a',  // valid byte character within 0..=255
    };

    let ast = ast::ClassSetItem::Literal(Box::new(x));

    translator_instance.push(HirFrame::ClassBytes(ClassBytes::empty()));
    translator_instance.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_byte_boundaries() {
    let mut stack: Vec<HirFrame> = Vec::new();
    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };
    let trans = Translator {
        stack: RefCell::new(stack),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let pattern = "boundary";
    let mut translator_instance = TranslatorI::new(&trans, pattern);

    for &c in &[0u8, 255u8] {
        let x = ast::Literal {
            span: Span { start: Position::new(0), end: Position::new(1) },
            kind: ast::LiteralKind::Byte,
            c: c as char,  // testing boundary characters
        };

        let ast = ast::ClassSetItem::Literal(Box::new(x));
        translator_instance.push(HirFrame::ClassBytes(ClassBytes::empty()));
        translator_instance.visit_class_set_item_post(&ast).unwrap();
    }
}

#[test]
fn test_visit_class_set_item_post_valid_chars() {
    let mut stack: Vec<HirFrame> = Vec::new();
    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };
    let trans = Translator {
        stack: RefCell::new(stack),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let pattern = "valid";
    let mut translator_instance = TranslatorI::new(&trans, pattern);

    for c in b'a'..=b'z' {
        let x = ast::Literal {
            span: Span { start: Position::new(0), end: Position::new(1) },
            kind: ast::LiteralKind::Byte,
            c: c as char,  // valid byte character from a to z
        };

        let ast = ast::ClassSetItem::Literal(Box::new(x));
        translator_instance.push(HirFrame::ClassBytes(ClassBytes::empty()));
        translator_instance.visit_class_set_item_post(&ast).unwrap();
    }
}

