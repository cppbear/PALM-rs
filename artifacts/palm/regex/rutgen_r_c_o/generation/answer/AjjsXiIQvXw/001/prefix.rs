// Answer 0

#[test]
fn test_is_valid_start_less_than_end() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let end_literal = Literal { span: span, kind: LiteralKind::Unicode('b'), c: 'b' };
    let class_set_range = ClassSetRange { span: span, start: start_literal, end: end_literal };
    let _ = class_set_range.is_valid();
}

#[test]
fn test_is_valid_start_equal_end() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let end_literal = Literal { span: span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let class_set_range = ClassSetRange { span: span, start: start_literal, end: end_literal };
    let _ = class_set_range.is_valid();
}

#[test]
fn test_is_valid_start_greater_than_end() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' };
    let end_literal = Literal { span: span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let class_set_range = ClassSetRange { span: span, start: start_literal, end: end_literal };
    let _ = class_set_range.is_valid();
}

#[test]
fn test_is_valid_unicode_boundary_low() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode(char::from(0)), c: char::from(0) };
    let end_literal = Literal { span: span, kind: LiteralKind::Unicode(char::from(1114111)), c: char::from(1114111) };
    let class_set_range = ClassSetRange { span: span, start: start_literal, end: end_literal };
    let _ = class_set_range.is_valid();
}

#[test]
fn test_is_valid_unicode_boundary_high() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode(char::from(1114111)), c: char::from(1114111) };
    let end_literal = Literal { span: span, kind: LiteralKind::Unicode(char::from(1114111)), c: char::from(1114111) };
    let class_set_range = ClassSetRange { span: span, start: start_literal, end: end_literal };
    let _ = class_set_range.is_valid();
}

