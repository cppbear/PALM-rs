// Answer 0

#[test]
fn test_parse_empty_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Empty(_) => assert!(true),
            _ => panic!("Expected an empty AST"),
        }
    }
}

#[test]
fn test_parse_single_literal() {
    let mut parser = Parser::new();
    let result = parser.parse("a");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Literal(_) => assert!(true),
            _ => panic!("Expected a Literal AST"),
        }
    }
}

#[test]
fn test_parse_grouped_expression() {
    let mut parser = Parser::new();
    let result = parser.parse("(abc)");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Group(_) => assert!(true),
            _ => panic!("Expected a Group AST"),
        }
    }
}

#[test]
fn test_parse_with_comments() {
    let mut parser = Parser::new();
    let result = parser.parse("# this is a comment\na");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Literal(_) => assert!(true),
            _ => panic!("Expected a Literal AST after comment"),
        }
    }
}

#[test]
fn test_parse_invalid_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("(*)");
    assert!(result.is_err());
    // Verify the error contains a relevant message or kind if possible
}

#[test]
fn test_parse_complex_expression() {
    let mut parser = Parser::new();
    let result = parser.parse("a(b|c)*");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Repetition(_) => assert!(true),
            _ => panic!("Expected a Repetition AST"),
        }
    }
}

