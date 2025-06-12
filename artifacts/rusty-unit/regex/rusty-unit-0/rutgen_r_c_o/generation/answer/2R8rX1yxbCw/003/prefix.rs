// Answer 0

#[test]
fn test_parse_simple_alternation() {
    let mut parser = Parser::new();
    let pattern = "(abc|def)";
    parser.parse(pattern);
}

#[test]
fn test_parse_repetition() {
    let mut parser = Parser::new();
    let pattern = "a*b+";
    parser.parse(pattern);
}

#[test]
fn test_parse_character_class() {
    let mut parser = Parser::new();
    let pattern = "[a-z]";
    parser.parse(pattern);
}

#[test]
fn test_parse_case_insensitive() {
    let mut parser = Parser::new();
    let pattern = "(?i)abc";
    parser.parse(pattern);
}

#[test]
fn test_parse_dot() {
    let mut parser = Parser::new();
    let pattern = ".*";
    parser.parse(pattern);
}

#[test]
fn test_parse_nested_groups() {
    let mut parser = Parser::new();
    let pattern = "((abc)|(def))?ghi";
    parser.parse(pattern);
}

#[test]
fn test_parse_empty_string() {
    let mut parser = Parser::new();
    let pattern = "";
    parser.parse(pattern);
}

#[test]
fn test_parse_complex_pattern() {
    let mut parser = Parser::new();
    let pattern = "(abc|def|ghi|jkl)*[0-9]+";
    parser.parse(pattern);
}

