// Answer 0

#[test]
fn test_parse_empty_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Empty(_) => {}
            _ => panic!("Expected an Ast::Empty variant"),
        }
    }
}

#[test]
fn test_parse_literal_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("a");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Literal(_) => {}
            _ => panic!("Expected an Ast::Literal variant"),
        }
    }
}

#[test]
fn test_parse_group_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("(a|b)");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Group(_) => {}
            _ => panic!("Expected an Ast::Group variant"),
        }
    }
}

#[test]
fn test_parse_invalid_pattern() {
    let mut parser = Parser::new();
    let result = parser.parse("a(b");
    assert!(result.is_err());
}

#[test]
fn test_parse_whitespace_ignored() {
    let mut parser = Parser::new();
    parser.ignore_whitespace.set(true);
    let result = parser.parse(" a b ");
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Ast::Concat(_) => {}
            _ => panic!("Expected an Ast::Concat variant"),
        }
    }
}

