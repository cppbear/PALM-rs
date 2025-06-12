// Answer 0

#[test]
fn test_parse_with_comments_empty_pattern() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(with_comments.ast.is_empty());
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_single_literal() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    match with_comments.ast {
        Ast::Concat { asts } => {
            assert_eq!(asts.len(), 1);
            if let Ast::Literal(_) = &asts[0] {
                // Literal matched
            } else {
                panic!("Expected a literal");
            }
        }
        _ => panic!("Expected a concatenation"),
    }
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_class() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("[a-z]");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    match with_comments.ast {
        Ast::Concat { asts } => {
            assert_eq!(asts.len(), 1);
            if let Ast::Class(_) = &asts[0] {
                // Class matched
            } else {
                panic!("Expected a class");
            }
        }
        _ => panic!("Expected a concatenation"),
    }
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_group() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("(a|b)");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    match with_comments.ast {
        Ast::Concat { asts } => {
            assert_eq!(asts.len(), 3);
            if let Ast::Group(_) = &asts[0] {
                // Group matched
            } else {
                panic!("Expected a group");
            }
        }
        _ => panic!("Expected a concatenation"),
    }
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_repetition() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a{2,3}");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    match with_comments.ast {
        Ast::Concat { asts } => {
            assert_eq!(asts.len(), 1);
            if let Ast::Repetition(_) = &asts[0] {
                // Repetition matched
            } else {
                panic!("Expected a repetition");
            }
        }
        _ => panic!("Expected a concatenation"),
    }
    assert!(with_comments.comments.is_empty());
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_multiple_calls() {
    let mut parser = Parser::new();
    let _ = parser.parse_with_comments("a");
    let _ = parser.parse_with_comments("b"); // This should panic
}

#[test]
fn test_parse_with_comments_until_eof() {
    let mut parser = Parser::new();
    parser.bump_space(); // As an example to simulate non-eof situation.
    let result = parser.parse_with_comments("a(b(c|d)e)?");
    assert!(result.is_ok());
}

#[test]
fn test_parse_with_comments_invalid_pattern() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a(b*c");
    assert!(result.is_err()); // This should return an error due to invalid pattern
}

