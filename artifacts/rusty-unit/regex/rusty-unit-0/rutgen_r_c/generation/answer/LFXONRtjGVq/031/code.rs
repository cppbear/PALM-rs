// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let pattern = "{2,5}"; // Valid counted repetition
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    
    let concat = Concat {
        span: Span::new(start_pos, end_pos),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, end_pos) })],
    };

    let parser = ParserI {
        parser: Parser { /* Mock/Stub parser state here based on actual implementation */ },
        pattern: pattern,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_invalid_unclosed() {
    let pattern = "{2,5"; // Missing closing brace
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    
    let concat = Concat {
        span: Span::new(start_pos, start_pos), // Invalid span since concat is not properly formed
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, start_pos) })],
    };

    let parser = ParserI {
        parser: Parser { /* Mock/Stub parser state here based on actual implementation */ },
        pattern: pattern,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_boundary_case_empty() {
    let pattern = "{}"; // Empty range
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 2, line: 1, column: 3 };

    let concat = Concat {
        span: Span::new(start_pos, end_pos),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, end_pos) })],
    };

    let parser = ParserI {
        parser: Parser { /* Mock/Stub parser state here based on actual implementation */ },
        pattern: pattern,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_valid_greedy_case() {
    let pattern = "{2,5}?"; // Valid counted repetition followed by '?'
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 6, line: 1, column: 7 };

    let concat = Concat {
        span: Span::new(start_pos, end_pos),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, end_pos) })],
    };

    let parser = ParserI {
        parser: Parser { /* Mock/Stub parser state here based on actual implementation */ },
        pattern: pattern,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_invalid_count_range() {
    let pattern = "{5,2}"; // Invalid range where start > end
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 6, line: 1, column: 7 };

    let concat = Concat {
        span: Span::new(start_pos, end_pos),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, end_pos) })],
    };

    let parser = ParserI {
        parser: Parser { /* Mock/Stub parser state here based on actual implementation */ },
        pattern: pattern,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

