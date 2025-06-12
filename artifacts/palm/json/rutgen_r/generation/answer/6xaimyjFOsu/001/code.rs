// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct Delegate;

impl Delegate {
    fn peek_position(&self) -> Position {
        Position { line: 1, column: 20 }
    }
}

struct Peeker {
    delegate: Delegate,
}

impl Peeker {
    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }
}

#[test]
fn test_peek_position_basic() {
    let peeker = Peeker { delegate: Delegate };
    let position = peeker.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 20);
}

#[test]
fn test_peek_position_edge_case() {
    let peeker = Peeker { delegate: Delegate };
    let position = peeker.peek_position();
    assert!(position.line > 0); // Check that line is positive
    assert!(position.column >= 0); // Check that column is non-negative
}

