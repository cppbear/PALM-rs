// Answer 0

#[test]
fn test_parser_builder_new_default() {
    let builder = ParserBuilder::new();
}

#[test]
fn test_parser_builder_nest_limit_zero() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(0);
}

#[test]
fn test_parser_builder_nest_limit_max() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(250);
}

#[test]
fn test_parser_builder_nest_limit_above_max() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(251);
}

#[test]
fn test_parser_builder_with_octal_true() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
}

#[test]
fn test_parser_builder_with_octal_false() {
    let mut builder = ParserBuilder::new();
    builder.octal(false);
}

#[test]
fn test_parser_builder_ignore_whitespace_true() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
}

#[test]
fn test_parser_builder_ignore_whitespace_false() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(false);
}

