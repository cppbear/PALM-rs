// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_star() {
    let pattern = "a*";
    let pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos, pos);
    
    let concat = ast::Concat {
        span,
        asts: vec![Ast::Literal(ast::Literal { span })]
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);

    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_with_plus() {
    let pattern = "b+";
    let pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos, pos);
    
    let concat = ast::Concat {
        span,
        asts: vec![Ast::Literal(ast::Literal { span })]
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);

    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_empty_concat() {
    let pattern = "c*";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(pos, pos);
    
    let concat = ast::Concat {
        span,
        asts: vec![]
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);

    assert!(result.is_err());
} 

#[test]
fn test_parse_uncounted_repetition_invalid_ast() {
    let pattern = "d*";
    let pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos, pos);
    
    let concat = ast::Concat {
        span,
        asts: vec![Ast::Empty(span)]
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);

    assert!(result.is_err());
} 

#[test]
fn test_parse_uncounted_repetition_greedy_and_non_greedy() {
    let pattern = "e+?";
    let pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos, pos);
    
    let concat = ast::Concat {
        span,
        asts: vec![Ast::Literal(ast::Literal { span })]
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);

    assert!(result.is_ok());
}

