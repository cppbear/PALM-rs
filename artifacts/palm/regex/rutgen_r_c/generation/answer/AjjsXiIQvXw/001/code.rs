// Answer 0

#[test]
fn test_class_set_range_valid() {
    let span = Span { start: Position(0), end: Position(1) };
    let start_literal = Literal::Unicode('a');
    let end_literal = Literal::Unicode('z');
    let class_set_range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    assert!(class_set_range.is_valid());
}

#[test]
fn test_class_set_range_invalid_start_greater_than_end() {
    let span = Span { start: Position(0), end: Position(1) };
    let start_literal = Literal::Unicode('z');
    let end_literal = Literal::Unicode('a');
    let class_set_range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    assert!(!class_set_range.is_valid());
}

#[test]
fn test_class_set_range_equal_start_and_end() {
    let span = Span { start: Position(0), end: Position(1) };
    let start_literal = Literal::Unicode('m');
    let end_literal = Literal::Unicode('m');
    let class_set_range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    assert!(class_set_range.is_valid());
}

#[test]
fn test_class_set_range_byte_literals() {
    let span = Span { start: Position(0), end: Position(1) };
    let start_literal = Literal::Byte(1);
    let end_literal = Literal::Byte(5);
    let class_set_range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    assert!(class_set_range.is_valid());
}

#[test]
fn test_class_set_range_byte_literal_invalid() {
    let span = Span { start: Position(0), end: Position(1) };
    let start_literal = Literal::Byte(5);
    let end_literal = Literal::Byte(1);
    let class_set_range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    assert!(!class_set_range.is_valid());
}

