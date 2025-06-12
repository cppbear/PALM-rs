// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct Delegate {
    position: Position,
}

impl Delegate {
    fn peek_position(&self) -> Position {
        self.position.clone()
    }
}

struct Reader {
    delegate: Delegate,
}

impl Reader {
    fn new(delegate: Delegate) -> Self {
        Reader { delegate }
    }

    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }
}

#[test]
fn test_peek_position_valid() {
    let delegate = Delegate { position: Position { line: 1, column: 5 } };
    let reader = Reader::new(delegate);
    let pos = reader.peek_position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_peek_position_boundary() {
    let delegate = Delegate { position: Position { line: 0, column: 0 } };
    let reader = Reader::new(delegate);
    let pos = reader.peek_position();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_peek_position_large_values() {
    let delegate = Delegate { position: Position { line: usize::max_value(), column: usize::max_value() } };
    let reader = Reader::new(delegate);
    let pos = reader.peek_position();
    assert_eq!(pos.line, usize::max_value());
    assert_eq!(pos.column, usize::max_value());
}

