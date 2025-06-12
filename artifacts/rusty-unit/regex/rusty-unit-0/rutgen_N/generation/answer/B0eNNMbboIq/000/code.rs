// Answer 0

#[derive(Debug)]
struct Span {
    start: Position,
    end: Position,
}

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl Span {
    pub fn is_one_line(&self) -> bool {
        self.start.line == self.end.line
    }
}

#[test]
fn test_is_one_line_same_line() {
    let span = Span {
        start: Position { line: 1, column: 5 },
        end: Position { line: 1, column: 10 },
    };
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_different_lines() {
    let span = Span {
        start: Position { line: 1, column: 5 },
        end: Position { line: 2, column: 10 },
    };
    assert!(!span.is_one_line());
}

#[test]
fn test_is_one_line_edge_case_start_end_same() {
    let span = Span {
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 1 },
    };
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_edge_case_same_column_different_lines() {
    let span = Span {
        start: Position { line: 1, column: 5 },
        end: Position { line: 3, column: 5 },
    };
    assert!(!span.is_one_line());
}

