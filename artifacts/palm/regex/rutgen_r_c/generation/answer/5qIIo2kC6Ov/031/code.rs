// Answer 0

#[test]
fn test_parse_with_comments_valid_input() {
    let pattern = r"abc|def";
    
    let mut parser = Parser::new();
    let result = parser.parse_with_comments(pattern);
    
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_nested_group() {
    let pattern = r"(abc|def)";
    
    let mut parser = Parser::new();
    let result = parser.parse_with_comments(pattern);
    
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Group { .. }));
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_multiple_calls() {
    let pattern = r"abc|def";
    
    let mut parser = Parser::new();
    let _ = parser.parse_with_comments(pattern);
    // This second call should panic
    let _ = parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_repetition() {
    let pattern = r"a{2,3}|b";
    
    let mut parser = Parser::new();
    let result = parser.parse_with_comments(pattern);
    
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_eof_condition() {
    let pattern = r"abc|";
    
    let mut parser = Parser::new();
    let result = parser.parse_with_comments(pattern);
    
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

