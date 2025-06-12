// Answer 0

#[test]
fn test_parser_creation_default() {
    let parser = Parser::new();
}

#[test]
fn test_parser_creation_with_nest_limit() {
    let builder = ParserBuilder::new().nest_limit(250);
    let parser = builder.build();
}

#[test]
fn test_parser_creation_with_octal_enabled() {
    let builder = ParserBuilder::new().octal(true);
    let parser = builder.build();
}

#[test]
fn test_parser_creation_with_whitespace_ignored() {
    let builder = ParserBuilder::new().ignore_whitespace(true);
    let parser = builder.build();
}

#[test]
fn test_parser_creation_with_max_nest_limit() {
    let builder = ParserBuilder::new().nest_limit(500);
    let parser = builder.build();
}

#[test]
fn test_parser_creation_with_zero_capture_index() {
    let parser = Parser::new();
    parser.capture_index.set(0);
}

#[test]
fn test_parser_creation_with_max_capture_index() {
    let parser = Parser::new();
    parser.capture_index.set(100);
}

#[test]
fn test_parser_creation_with_position_limits() {
    let parser = Parser::new();
    parser.pos.set(Position { offset: 0, line: 1, column: 1 });
    parser.pos.set(Position { offset: 1000, line: 100, column: 100 });
}

#[test]
fn test_parser_creation_with_octal_disabled() {
    let builder = ParserBuilder::new().octal(false);
    let parser = builder.build();
}

