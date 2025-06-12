// Answer 0

#[derive(Debug, Clone, Copy)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
struct Span {
    start: Position,
    end: Position,
}

#[test]
fn test_from_formatter_with_aux_span() {
    let pattern = "a\n";
    let span = Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 1 } };
    let aux_span = Some(span.clone());
    
    let formatter = Formatter {
        pattern,
        err: &"error",
        span: &span,
        aux_span,
    };
    
    let spans = Spans::from_formatter(&formatter);
}

