// Answer 0

#[test]
fn test_parser_default() {
    let parser = Parser::new();
}

#[test]
fn test_parser_with_nest_limit_zero() {
    let parser_builder = ParserBuilder::new().nest_limit(0);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_nest_limit_max() {
    let parser_builder = ParserBuilder::new().nest_limit(100);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_octal_true() {
    let parser_builder = ParserBuilder::new().octal(true);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_octal_false() {
    let parser_builder = ParserBuilder::new().octal(false);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_allow_invalid_utf8_true() {
    let parser_builder = ParserBuilder::new().allow_invalid_utf8(true);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_allow_invalid_utf8_false() {
    let parser_builder = ParserBuilder::new().allow_invalid_utf8(false);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_ignore_whitespace_true() {
    let parser_builder = ParserBuilder::new().ignore_whitespace(true);
    let parser = parser_builder.build();
}

#[test]
fn test_parser_with_ignore_whitespace_false() {
    let parser_builder = ParserBuilder::new().ignore_whitespace(false);
    let parser = parser_builder.build();
}

