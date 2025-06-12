// Answer 0

#[test]
fn class_set_item_literal_pop_some_class_bytes_class_literal_byte_err() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassBytes(ClassBytes::empty())]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let literal = Literal { span: Span { start: Position { value: 0 }, end: Position { value: 1 } }, kind: LiteralKind::Character, c: 'A' };
    let ast_item = ast::ClassSetItem::Literal(literal);
    let result = translator.visit_class_set_item_post(&ast_item);
}

#[test]
fn class_set_item_literal_pop_none_class_bytes_class_literal_byte_err() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let literal = Literal { span: Span { start: Position { value: 0 }, end: Position { value: 1 } }, kind: LiteralKind::Character, c: 'A' };
    let ast_item = ast::ClassSetItem::Literal(literal);
    let result = translator.visit_class_set_item_post(&ast_item);
}

#[test]
fn class_set_item_literal_pop_some_class_unicode_class_literal_byte_err() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassUnicode(ClassUnicode::empty())]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let literal = Literal { span: Span { start: Position { value: 0 }, end: Position { value: 1 } }, kind: LiteralKind::Character, c: 'A' };
    let ast_item = ast::ClassSetItem::Literal(literal);
    let result = translator.visit_class_set_item_post(&ast_item);
}

#[test]
fn class_set_item_literal_pop_some_class_bytes_class_literal_byte_none() {
    let mut translator = Translator {
        stack: RefCell::new(vec![HirFrame::ClassBytes(ClassBytes::empty())]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let literal = Literal { span: Span { start: Position { value: 0 }, end: Position { value: 1 } }, kind: LiteralKind::Character, c: 'B' };
    let ast_item = ast::ClassSetItem::Literal(literal);
    let result = translator.visit_class_set_item_post(&ast_item);
}

#[test]
fn class_set_item_literal_pop_none_class_bytes_class_literal_byte_none() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let literal = Literal { span: Span { start: Position { value: 0 }, end: Position { value: 1 } }, kind: LiteralKind::Character, c: 'B' };
    let ast_item = ast::ClassSetItem::Literal(literal);
    let result = translator.visit_class_set_item_post(&ast_item);
}

