// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct LineColIterator {
    current_position: Position,
}

impl LineColIterator {
    fn new() -> Self {
        LineColIterator {
            current_position: Position { line: 1, column: 0 },
        }
    }

    fn position(&self) -> Position {
        self.current_position.clone()
    }

    fn peek_position(&self) -> Position {
        self.position()
    }
}

#[test]
fn test_peek_position_initial() {
    let iterator = LineColIterator::new();
    let position = iterator.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
fn test_peek_position_after_some_iterations() {
    let mut iterator = LineColIterator::new();
    iterator.current_position.line += 1; // Simulate moving to the next line
    iterator.current_position.column += 5; // Simulate moving to the right
    let position = iterator.peek_position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 5);
}

#[test]
#[should_panic]
fn test_peek_position_no_panic() {
    let iterator = LineColIterator::new();
    let position = iterator.peek_position(); // This should not panic
}

