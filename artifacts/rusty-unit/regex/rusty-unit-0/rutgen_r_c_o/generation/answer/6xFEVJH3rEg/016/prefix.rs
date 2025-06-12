// Answer 0

#[test]
fn test_fmt_empty_pattern() {
    let pattern = "";
    let err = "Syntax error";
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 1 } };
    let formatter = Formatter { pattern, err, span, aux_span: None };
    let mut output = String::new();
    let mut f = &mut output;
    let _ = formatter.fmt(&mut f);
}

#[test]
fn test_fmt_single_character_pattern() {
    let pattern = "a";
    let err = "Single character syntax error";
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 2 } };
    let formatter = Formatter { pattern, err, span, aux_span: None };
    let mut output = String::new();
    let mut f = &mut output;
    let _ = formatter.fmt(&mut f);
}

#[test]
fn test_fmt_multiple_character_pattern() {
    let pattern = "abc";
    let err = "Multiple character syntax error";
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 4 } };
    let formatter = Formatter { pattern, err, span, aux_span: None };
    let mut output = String::new();
    let mut f = &mut output;
    let _ = formatter.fmt(&mut f);
}

#[test]
fn test_fmt_single_line_pattern_with_aux() {
    let pattern = "a+b";
    let err = "Quantifier syntax error";
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 4 } };
    let aux_span = ast::Span { start: Position { line: 1, column: 2 }, end: Position { line: 1, column: 3 } };
    let formatter = Formatter { pattern, err, span, aux_span: Some(&aux_span) };
    let mut output = String::new();
    let mut f = &mut output;
    let _ = formatter.fmt(&mut f);
}

