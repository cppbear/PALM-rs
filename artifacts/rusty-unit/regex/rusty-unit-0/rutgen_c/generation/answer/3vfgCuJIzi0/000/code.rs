// Answer 0

#[test]
fn test_span_debug_fmt() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let span = Span { start, end };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", span);
    assert!(result.is_ok());
    assert_eq!(output, "Span(Position { offset: 0, line: 1, column: 1 }, Position { offset: 5, line: 1, column: 6 })");
}

