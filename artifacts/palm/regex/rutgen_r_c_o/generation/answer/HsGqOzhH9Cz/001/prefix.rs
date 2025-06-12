// Answer 0

#[test]
fn test_parse_empty_pattern() {
    let mut parser = Parser::new();
    parser.capture_index.set(0);
    parser.nest_limit = 5;
    let _ = parser.parse("");
}

#[test]
fn test_parse_single_character() {
    let mut parser = Parser::new();
    parser.capture_index.set(1);
    parser.nest_limit = 5;
    let _ = parser.parse("a");
}

#[test]
fn test_parse_multiple_characters() {
    let mut parser = Parser::new();
    parser.capture_index.set(2);
    parser.nest_limit = 5;
    let _ = parser.parse("abc");
}

#[test]
fn test_parse_special_characters() {
    let mut parser = Parser::new();
    parser.capture_index.set(3);
    parser.nest_limit = 5;
    let _ = parser.parse(".?*+");
}

#[test]
fn test_parse_with_whitespace() {
    let mut parser = Parser::new();
    parser.capture_index.set(1);
    parser.nest_limit = 5;
    let _ = parser.parse("a b c");
}

#[test]
fn test_parse_long_pattern() {
    let mut parser = Parser::new();
    parser.capture_index.set(4);
    parser.nest_limit = 5;
    let _ = parser.parse("a".repeat(100).as_str());
}

#[test]
fn test_parse_nested_groups() {
    let mut parser = Parser::new();
    parser.capture_index.set(2);
    parser.nest_limit = 10;
    let _ = parser.parse("(a(b(c)))");
}

#[test]
fn test_parse_exceed_nest_limit() {
    let mut parser = Parser::new();
    parser.capture_index.set(0);
    parser.nest_limit = 1;
    let _ = parser.parse("(a(b(c)))");
}

#[test]
fn test_parse_with_capture_names() {
    let mut parser = Parser::new();
    parser.capture_index.set(5);
    parser.nest_limit = 5;
    let _ = parser.parse("(?<name>abc)");
}

