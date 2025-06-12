// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    
    let ast = Ast::Literal(ast::Literal { span });
    let mut concat = ast::Concat {
        span,
        asts: vec![ast],
    };
    
    let parser_state = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI {
        parser: &parser_state,
        pattern: "?",
    };
    
    let result = parser_instance.parse_uncounted_repetition(concat.clone(), ast::RepetitionKind::ZeroOrMore);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_with_asterisk() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    
    let ast = Ast::Literal(ast::Literal { span });
    let mut concat = ast::Concat {
        span,
        asts: vec![ast],
    };

    let parser_state = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI {
        parser: &parser_state,
        pattern: "*",
    };

    let result = parser_instance.parse_uncounted_repetition(concat.clone(), ast::RepetitionKind::ZeroOrMore);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_uncounted_repetition_with_empty_concat() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    
    let mut concat = ast::Concat {
        span,
        asts: vec![],
    };

    let parser_state = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI {
        parser: &parser_state,
        pattern: "?",
    };
    
    let _result = parser_instance.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

#[test]
#[should_panic]
fn test_parse_uncounted_repetition_with_invalid_ast() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);

    let ast = Ast::Flags(ast::Flags { span });
    let mut concat = ast::Concat {
        span,
        asts: vec![ast],
    };

    let parser_state = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI {
        parser: &parser_state,
        pattern: "*",
    };
    
    let _result = parser_instance.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

