// Answer 0

#[test]
fn test_octal_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(false);
}

#[test]
fn test_octal_with_nest_limit() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(50).octal(true);
}

#[test]
fn test_octal_with_max_nest_limit() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(100).octal(false);
}

#[test]
fn test_octal_with_min_nest_limit() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(0).octal(true);
}

#[test]
fn test_octal_after_setting_whitespace() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(true).octal(true);
}

