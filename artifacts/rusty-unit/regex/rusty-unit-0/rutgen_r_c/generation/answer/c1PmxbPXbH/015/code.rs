// Answer 0

#[test]
fn test_parse_uncounted_repetition_question_mark() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position_start, position_end);
    
    let ast = Ast::Literal(ast::Literal { span: span });
    let mut concat = ast::Concat { span: span, asts: vec![ast] };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position_end),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "*",
    };
    
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_star() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position_start, position_end);
    
    let ast = Ast::Literal(ast::Literal { span: span });
    let mut concat = ast::Concat { span: span, asts: vec![ast] };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position_end),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "*",
    };
    
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_plus() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position_start, position_end);
    
    let ast = Ast::Literal(ast::Literal { span: span });
    let mut concat = ast::Concat { span: span, asts: vec![ast] };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position_end),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "+",
    };
    
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);
    assert!(result.is_ok());
}

