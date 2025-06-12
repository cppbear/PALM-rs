// Answer 0

#[test]
fn test_parse_counted_repetition_valid_exact() {
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }), value: 'a' })],
    };
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
            capture_index: Cell::new(0), 
            nest_limit: 0, 
            octal: false, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new(String::from("{3}")),
        },
        pattern: "{3}",
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_valid_bounded() {
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }), value: 'a' })],
    };
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
            capture_index: Cell::new(0), 
            nest_limit: 0, 
            octal: false, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new(String::from("{2,5}")),
        },
        pattern: "{2,5}",
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_invalid_missing_expression() {
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![],
    };
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
            capture_index: Cell::new(0), 
            nest_limit: 0, 
            octal: false, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new(String::from("{2}")),
        },
        pattern: "{2}",
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_invalid_unclosed() {
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }), value: 'a' })],
    };
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
            capture_index: Cell::new(0), 
            nest_limit: 0, 
            octal: false, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new(String::from("{2")),
        },
        pattern: "{2",
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

