// Answer 0

#[test]
fn test_parse_with_comments_valid_pattern() {
    let mut parser = Parser::new();
    let pattern = r"(?i)foo # This is a comment\nbar";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast.to_string(), "foo|bar");
    assert_eq!(with_comments.comments.len(), 1);
    assert_eq!(with_comments.comments[0].comment, " This is a comment");
}

#[test]
fn test_parse_with_comments_empty_pattern() {
    let mut parser = Parser::new();
    let pattern = "";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_err());
}

#[test]
fn test_parse_with_comments_only_comments() {
    let mut parser = Parser::new();
    let pattern = "# Only comment\n";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(with_comments.ast.is_empty());  // Assuming empty AST for comments only.
    assert_eq!(with_comments.comments.len(), 1);
    assert_eq!(with_comments.comments[0].comment, " Only comment");
}

#[test]
#[should_panic]
fn test_parse_with_comments_invalid_pattern() {
    let mut parser = Parser::new();
    let pattern = r"foo [bar";  // Missing closing bracket
    let _result = parser.parse_with_comments(pattern);  // This should panic
}

#[test]
fn test_parse_with_comments_mixed_content() {
    let mut parser = Parser::new();
    let pattern = r"foo (bar # Comment on bar\n|baz#Comment on baz)";
    let result = parser.parse_with_comments(pattern);
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast.to_string(), "foo|(bar|baz)"); // Simplified expected AST
    assert_eq!(with_comments.comments.len(), 2);
    assert_eq!(with_comments.comments[0].comment, " Comment on bar");
    assert_eq!(with_comments.comments[1].comment, "Comment on baz");
}

