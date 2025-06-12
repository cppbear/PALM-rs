// Answer 0

#[test]
fn test_multi_line_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(true);
}

#[test]
fn test_multi_line_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(false);
}

#[test]
fn test_multi_line_repeated_calls() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(true);
    parser_builder.multi_line(false);
    parser_builder.multi_line(true);
}

#[test]
fn test_multi_line_with_varied_initializations() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(true);
    parser_builder.nest_limit(10);
    parser_builder.multi_line(true);

    let mut another_builder = ParserBuilder::new();
    another_builder.octal(true);
    another_builder.multi_line(false);
}

