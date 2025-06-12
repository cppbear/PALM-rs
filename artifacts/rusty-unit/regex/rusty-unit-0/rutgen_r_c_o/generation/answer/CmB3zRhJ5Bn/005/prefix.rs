// Answer 0

#[test]
fn test_into_ast_literal_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Unicode, c: 'a' };
    let primitive = Primitive::Literal(lit);
    primitive.into_ast();
}

#[test]
fn test_into_ast_literal_byte() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Byte, c: 'b' as u8 };
    let primitive = Primitive::Literal(lit);
    primitive.into_ast();
}

#[test]
fn test_into_ast_literal_special_character() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Unicode, c: '©' };
    let primitive = Primitive::Literal(lit);
    primitive.into_ast();
}

#[test]
fn test_into_ast_literal_control_character() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Byte, c: '\0' as u8 };
    let primitive = Primitive::Literal(lit);
    primitive.into_ast();
}

#[test]
fn test_into_ast_literal_high_byte() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Byte, c: 0xFF as u8 };
    let primitive = Primitive::Literal(lit);
    primitive.into_ast();
}

