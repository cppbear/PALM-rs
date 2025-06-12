// Answer 0

#[test]
fn test_parse_with_comments_single_repetition() {
    use ast::{Ast, Class, RepetitionKind, Literal};

    let mut parser = Parser::new();
    let pattern = "a+";
    let result = parser.parse_with_comments(pattern);

    assert!(result.is_ok());
    let with_comments = result.unwrap();

    assert_eq!(with_comments.comments.len(), 0);
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_grouping() {
    use ast::{Ast, Class, RepetitionKind, Literal, Group};

    let mut parser = Parser::new();
    let pattern = "(a|b)+";
    let result = parser.parse_with_comments(pattern);

    assert!(result.is_ok());
    let with_comments = result.unwrap();

    assert_eq!(with_comments.comments.len(), 0);
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_class() {
    use ast::{Ast, Class};

    let mut parser = Parser::new();
    let pattern = "[a-z]+";
    let result = parser.parse_with_comments(pattern);

    assert!(result.is_ok());
    let with_comments = result.unwrap();

    assert_eq!(with_comments.comments.len(), 0);
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_counted_repetition() {
    use ast::{Ast, RepetitionKind};

    let mut parser = Parser::new();
    let pattern = "a{2,3}";
    let result = parser.parse_with_comments(pattern);

    assert!(result.is_ok());
    let with_comments = result.unwrap();

    assert_eq!(with_comments.comments.len(), 0);
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
}

#[test]
fn test_parse_with_comments_empty_pattern() {
    let mut parser = Parser::new();
    let pattern = "";
    let result = parser.parse_with_comments(pattern);

    assert!(result.is_err());
}

