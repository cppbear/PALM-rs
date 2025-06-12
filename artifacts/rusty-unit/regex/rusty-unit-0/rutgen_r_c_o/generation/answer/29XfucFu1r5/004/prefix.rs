// Answer 0

#[test]
fn test_into_class_set_item_literal_byte_0() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let lit = ast::Literal::Byte(0);
    let primitive = Primitive::Literal(lit.clone());
    let parser = ParserI::new((), "test_pattern");
    let _ = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_literal_byte_128() {
    let span = Span { start: Position::from(1), end: Position::from(2) };
    let lit = ast::Literal::Byte(128);
    let primitive = Primitive::Literal(lit.clone());
    let parser = ParserI::new((), "test_pattern");
    let _ = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_literal_byte_255() {
    let span = Span { start: Position::from(2), end: Position::from(3) };
    let lit = ast::Literal::Byte(255);
    let primitive = Primitive::Literal(lit.clone());
    let parser = ParserI::new((), "test_pattern");
    let _ = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_literal_unicode() {
    let span = Span { start: Position::from(3), end: Position::from(4) };
    let lit = ast::Literal::Unicode('a');
    let primitive = Primitive::Literal(lit);
    let parser = ParserI::new((), "test_pattern");
    let _ = primitive.into_class_set_item(&parser);
}

