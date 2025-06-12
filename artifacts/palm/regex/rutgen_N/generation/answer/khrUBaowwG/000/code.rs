// Answer 0

#[derive(Debug, PartialEq)]
struct Position {
    value: usize,
}

#[derive(Debug, PartialEq)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn with_start(self, pos: Position) -> Span {
        Span { start: pos, ..self }
    }
}

#[test]
fn test_with_start_changes_start_position() {
    let original_span = Span {
        start: Position { value: 1 },
        end: Position { value: 5 },
    };
    let new_position = Position { value: 3 };
    let updated_span = original_span.with_start(new_position);

    assert_eq!(updated_span.start, new_position);
    assert_eq!(updated_span.end, original_span.end);
}

#[test]
fn test_with_start_on_same_position() {
    let original_span = Span {
        start: Position { value: 2 },
        end: Position { value: 4 },
    };
    let same_position = Position { value: 2 };
    let updated_span = original_span.with_start(same_position);

    assert_eq!(updated_span.start, same_position);
    assert_eq!(updated_span.end, original_span.end);
}

#[test]
fn test_with_start_changes_start_position_to_end() {
    let original_span = Span {
        start: Position { value: 0 },
        end: Position { value: 10 },
    };
    let new_position = Position { value: 10 };
    let updated_span = original_span.with_start(new_position);

    assert_eq!(updated_span.start, new_position);
    assert_eq!(updated_span.end, original_span.end);
}

