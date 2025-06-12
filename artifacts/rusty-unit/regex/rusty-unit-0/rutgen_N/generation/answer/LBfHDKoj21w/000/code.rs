// Answer 0

#[derive(Debug, PartialEq)]
struct Position(usize);

#[derive(Debug, PartialEq)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }

    pub fn with_end(self, pos: Position) -> Span {
        Span { end: pos, ..self }
    }
}

#[test]
fn test_with_end_replaces_end_position() {
    let start_pos = Position(1);
    let end_pos = Position(5);
    let new_end_pos = Position(10);
    
    let span = Span::new(start_pos, end_pos);
    let updated_span = span.with_end(new_end_pos);
    
    assert_eq!(updated_span.end, new_end_pos);
    assert_eq!(updated_span.start, end_pos);
}

#[test]
fn test_with_end_does_not_change_start_position() {
    let start_pos = Position(3);
    let end_pos = Position(7);
    let new_end_pos = Position(9);
    
    let span = Span::new(start_pos, end_pos);
    let updated_span = span.with_end(new_end_pos);
    
    assert_eq!(updated_span.start, start_pos);
    assert_ne!(updated_span.end, end_pos);
}

#[test]
fn test_with_end_keeps_same_start_end_values() {
    let start_pos = Position(0);
    let end_pos = Position(1);
    
    let span = Span::new(start_pos, end_pos);
    let updated_span = span.with_end(end_pos.clone());
    
    assert_eq!(updated_span.start, start_pos);
    assert_eq!(updated_span.end, end_pos);
}

