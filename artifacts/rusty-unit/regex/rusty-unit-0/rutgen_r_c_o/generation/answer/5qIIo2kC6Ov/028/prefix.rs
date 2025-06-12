// Answer 0

#[test]
fn test_parse_with_comments_simple_alternation() {
    let mut parser = Parser::new();
    parser.nest_limit = 5;
    parser.capture_index.set(1);
    parser.ignore_whitespace.set(false);
    let pattern = "a|b|c";
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
#[should_panic]
fn test_parse_with_comments_exceed_nest_limit() {
    let mut parser = Parser::new();
    parser.nest_limit = 1; // Set to a low limit to trigger panic
    parser.capture_index.set(0);
    parser.ignore_whitespace.set(true);
    let pattern = "(((a|b)))"; // This will exceed the nesting limit
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
fn test_parse_with_comments_multiple_nesting() {
    let mut parser = Parser::new();
    parser.nest_limit = 3;
    parser.capture_index.set(2);
    parser.ignore_whitespace.set(true);
    let pattern = "(a|(b|c))";
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
#[should_panic]
fn test_parse_with_comments_empty_pattern() {
    let mut parser = Parser::new();
    parser.nest_limit = 5;
    parser.capture_index.set(0);
    parser.ignore_whitespace.set(false);
    let pattern = ""; // Edge case for empty pattern
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
fn test_parse_with_comments_alternation_with_comments() {
    let mut parser = Parser::new();
    parser.nest_limit = 5;
    parser.capture_index.set(3);
    parser.ignore_whitespace.set(true);
    let pattern = "a|b # this is a comment";
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
fn test_parse_with_comments_alternation_deep_nesting() {
    let mut parser = Parser::new();
    parser.nest_limit = 4;
    parser.capture_index.set(4);
    parser.ignore_whitespace.set(true);
    let pattern = "((a|(b|c))|d)";
    parser.parse_with_comments(pattern).unwrap();
}

#[test]
fn test_parse_with_comments_whitespace_ignored() {
    let mut parser = Parser::new();
    parser.nest_limit = 5;
    parser.capture_index.set(0);
    parser.ignore_whitespace.set(true);
    let pattern = "   a   |   b   |   c   ";
    parser.parse_with_comments(pattern).unwrap();
}

