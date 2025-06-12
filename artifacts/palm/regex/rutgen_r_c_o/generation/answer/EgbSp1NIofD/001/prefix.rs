// Answer 0

#[test]
fn test_parser_builder_default() {
    let builder = ParserBuilder::new();
    let parser = builder.build();
}

#[test]
fn test_parser_builder_nest_limit_zero() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(0);
    let parser = builder.build();
}

#[test]
fn test_parser_builder_nest_limit_max() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(1000);
    let parser = builder.build();
}

#[test]
fn test_parser_builder_octal_false() {
    let mut builder = ParserBuilder::new();
    builder.octal(false);
    let parser = builder.build();
}

#[test]
fn test_parser_builder_octal_true() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    let parser = builder.build();
}

#[test]
fn test_parser_builder_ignore_whitespace_false() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(false);
    let parser = builder.build();
}

#[test]
fn test_parser_builder_ignore_whitespace_true() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
    let parser = builder.build();
}

