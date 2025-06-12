// Answer 0

#[derive(Debug, PartialEq)]
struct Position(u32);

#[derive(Debug, PartialEq)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

#[test]
fn test_splat_with_zero_position() {
    let pos = Position(0);
    let expected = Span::new(Position(0), Position(0));
    assert_eq!(splat(pos), expected);
}

#[test]
fn test_splat_with_positive_position() {
    let pos = Position(5);
    let expected = Span::new(Position(5), Position(5));
    assert_eq!(splat(pos), expected);
}

#[test]
fn test_splat_with_large_position() {
    let pos = Position(u32::MAX);
    let expected = Span::new(Position(u32::MAX), Position(u32::MAX));
    assert_eq!(splat(pos), expected);
}

