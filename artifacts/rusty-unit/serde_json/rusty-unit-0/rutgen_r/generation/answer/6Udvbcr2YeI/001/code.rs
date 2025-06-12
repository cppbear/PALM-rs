// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct MockReader {
    pos: Position,
}

impl MockReader {
    fn new(line: usize, column: usize) -> Self {
        MockReader {
            pos: Position { line, column },
        }
    }
    
    fn position(&self) -> Position {
        self.pos.clone()
    }

    fn peek_position(&self) -> Position {
        self.position()
    }
}

#[test]
fn test_peek_position_normal_case() {
    let reader = MockReader::new(1, 10);
    let pos = reader.peek_position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 10);
}

#[test]
fn test_peek_position_zero_case() {
    let reader = MockReader::new(0, 0);
    let pos = reader.peek_position();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_peek_position_large_numbers() {
    let reader = MockReader::new(1000, 2000);
    let pos = reader.peek_position();
    assert_eq!(pos.line, 1000);
    assert_eq!(pos.column, 2000);
}

#[test]
fn test_peek_position_boundary() {
    let reader = MockReader::new(usize::MAX, usize::MAX);
    let pos = reader.peek_position();
    assert_eq!(pos.line, usize::MAX);
    assert_eq!(pos.column, usize::MAX);
}

