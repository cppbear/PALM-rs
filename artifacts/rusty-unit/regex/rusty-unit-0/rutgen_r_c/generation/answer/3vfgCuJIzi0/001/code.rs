// Answer 0

#[test]
fn test_span_fmt_debug() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 10, line: 1, column: 11 };
    let span = Span { start: position_start, end: position_end };
    
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        let result = span.fmt(formatter);
        assert!(result.is_ok());
    }

    assert_eq!(output, "Span(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 })");
}

#[test]
fn test_span_fmt_debug_same_start_end() {
    let position = Position { offset: 5, line: 2, column: 6 };
    let span = Span { start: position, end: position };
    
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        let result = span.fmt(formatter);
        assert!(result.is_ok());
    }

    assert_eq!(output, "Span(Position { offset: 5, line: 2, column: 6 }, Position { offset: 5, line: 2, column: 6 })");
}

#[test]
fn test_span_fmt_debug_large_values() {
    let position_start = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    let position_end = Position { offset: usize::MAX - 1, line: usize::MAX, column: usize::MAX - 1 };
    let span = Span { start: position_start, end: position_end };
    
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        let result = span.fmt(formatter);
        assert!(result.is_ok());
    }

    assert_eq!(output, "Span(Position { offset: 18446744073709551615, line: 18446744073709551615, column: 18446744073709551615 }, Position { offset: 18446744073709551614, line: 18446744073709551615, column: 18446744073709551614 })");
}

