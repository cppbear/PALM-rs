// Answer 0

#[test]
fn test_parse_with_comments_empty() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.comments.len(), 0);
}

#[test]
fn test_parse_with_comments_single_character_class() {
    let parser = Parser::new();
    let pattern = "[a-z]";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, ast::Ast::Class(_)));
    assert_eq!(with_comments.comments.len(), 0);
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_parser_used_once() {
    let parser = Parser::new();
    parser.parse_with_comments("abc").unwrap();
    // Attempt to call again should panic
    parser.parse_with_comments("xyz").unwrap();
}

#[test]
fn test_parse_with_comments_unclosed_class() {
    let parser = Parser::new();
    let pattern = "[a-z"; // Unclosed character class
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_err());
}

#[test]
fn test_parse_with_comments_invalid_repetition() {
    let parser = Parser::new();
    let pattern = "a{"; // Invalid repetition
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_err());
}

#[test]
fn test_parse_with_comments_multiple_segments() {
    let parser = Parser::new();
    let pattern = "(abc)|(def)";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, ast::Ast::Alternation(_)));
    assert_eq!(with_comments.comments.len(), 0);
}

