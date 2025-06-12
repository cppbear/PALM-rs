// Answer 0

#[test]
fn test_is_one_line_same_line() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: 1 },
        end: Position { line: 1 },
    };
    
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_different_lines() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: 1 },
        end: Position { line: 2 },
    };
    
    assert!(!span.is_one_line());
}

#[test]
fn test_is_one_line_boundary_start_zero() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: 0 },
        end: Position { line: 0 },
    };
    
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_boundary_end_zero() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: 0 },
        end: Position { line: 1 },
    };
    
    assert!(!span.is_one_line());
}

#[test]
fn test_is_one_line_large_numbers() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: usize::MAX },
        end: Position { line: usize::MAX },
    };
    
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_large_different_numbers() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
    }

    let span = Span {
        start: Position { line: usize::MAX - 1 },
        end: Position { line: usize::MAX },
    };

    assert!(!span.is_one_line());
}

