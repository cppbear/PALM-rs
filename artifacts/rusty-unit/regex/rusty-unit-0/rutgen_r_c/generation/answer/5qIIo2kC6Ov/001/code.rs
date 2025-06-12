// Answer 0

#[test]
fn test_parse_with_comments_base_case() {
    let parser = Parser::new();
    let parser_i = ParserI {
        parser: &parser,
        pattern: "(abc) | def",
    };
    
    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(
        with_comments.ast,
        Ast::Concat(ast::Concat {
            span: Span { start: 0, end: 13 },
            asts: vec![
                Ast::Group(ast::Group { /* fill appropriate fields */ }),
                Ast::Class(ast::Class::Bracketed(ast::ClassBracketed { /* fill appropriate fields */ }))
            ],
        })
    );
    assert!(with_comments.comments.is_empty());
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_multiple_calls() {
    let parser = Parser::new();
    let parser_i = ParserI {
        parser: &parser,
        pattern: "(abc)",
    };
    
    let _ = parser_i.parse_with_comments(); // First call
    let _ = parser_i.parse_with_comments(); // This should panic
}

#[test]
fn test_parse_with_comments_eof_condition() {
    let parser = Parser::new();
    let parser_i = ParserI {
        parser: &parser,
        pattern: "(xyz) |",
    };
    
    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(
        with_comments.ast,
        Ast::Concat(ast::Concat {
            span: Span { start: 0, end: 9 },
            asts: vec![Ast::Group(ast::Group { /* fill appropriate fields */ })],
        })
    );
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_pop_group_end_error() {
    let parser = Parser::new();
    let parser_i = ParserI {
        parser: &parser,
        pattern: "(abc|def) |", // Extra pipe to validate error on pop_group_end
    };
    
    let result = parser_i.parse_with_comments();
    assert!(result.is_err()); // Expecting an Err due to invalid syntax 
}

