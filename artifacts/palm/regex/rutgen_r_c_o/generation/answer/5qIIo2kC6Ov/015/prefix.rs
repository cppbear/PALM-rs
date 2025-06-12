// Answer 0

#[test]
fn test_parse_with_comments_simple_expression() {
    let mut parser = Parser::new();
    let pattern = "abc+";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_nested_group() {
    let mut parser = Parser::new();
    let pattern = "(abc|def)+";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_character_class() {
    let mut parser = Parser::new();
    let pattern = "[a-z]+";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_capture_group() {
    let mut parser = Parser::new();
    let pattern = "(a(bc)+)+";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_repetition() {
    let mut parser = Parser::new();
    let pattern = "(abc){2,4}";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_large_pattern() {
    let mut parser = Parser::new();
    let pattern = "a{1,100}(b|c)*[1-9]+";
    parser.parse_with_comments(pattern);
}

