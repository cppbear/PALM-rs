// Answer 0

#[test]
fn test_into_class_literal_valid_unicode_char() {
    let pattern = "^[a-zA-Z]+$";
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Unicode('a');
    let primitive = Primitive::Literal(ast::Literal { span, kind: LiteralKind::Char, c: 'a' });
    let result = primitive.into_class_literal(&parser);
}

#[test]
fn test_into_class_literal_valid_byte() {
    let pattern = "^[a-zA-Z]+$";
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Byte(0x61);
    let primitive = Primitive::Literal(ast::Literal { span, kind: LiteralKind::Byte, c: 'a' });
    let result = primitive.into_class_literal(&parser);
}

#[test]
fn test_into_class_literal_valid_unicode_scalar() {
    let pattern = "^[\u{1F600}-\u{1F64F}]$"; // Emoji range
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Unicode('😀'); // Grinning Face
    let primitive = Primitive::Literal(ast::Literal { span, kind: LiteralKind::Char, c: '😀' });
    let result = primitive.into_class_literal(&parser);
}

#[test]
fn test_into_class_literal_unicode_boundary() {
    let pattern = "^[\u{10FFFF}]$"; // Upper boundary of Unicode
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Unicode('\u{10FFFF}');
    let primitive = Primitive::Literal(ast::Literal { span, kind: LiteralKind::Char, c: '\u{10FFFF}' });
    let result = primitive.into_class_literal(&parser);
} 

#[test]
fn test_into_class_literal_zero_char() {
    let pattern = "^[\u{0000}]$"; // Lower boundary of Unicode
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Unicode('\u{0000}');
    let primitive = Primitive::Literal(ast::Literal { span, kind: LiteralKind::Char, c: '\u{0000}' });
    let result = primitive.into_class_literal(&parser);
} 

#[test]
#[should_panic]
fn test_into_class_literal_invalid_case() {
    let pattern = "^[a-zA-Z]+$";
    let parser = ParserI::new(RefCell::new(()), pattern);
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span); // Not a valid literal
    let result = primitive.into_class_literal(&parser);
}

