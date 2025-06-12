// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_range() {
    let pattern = "{1,2";
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    
    let mut concat = ast::Concat {
        span: span_start,
        asts: vec![Ast::Literal(ast::Literal { span: span_start, value: 'a' })],
    };

    let parser = ParserI {
        parser: Parser {
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
            scratch: RefCell::new(String::from("")),
        },
        pattern,
    };

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_empty_repetition() {
    let pattern = "{";
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    
    let mut concat = ast::Concat {
        span: span_start,
        asts: vec![Ast::Literal(ast::Literal { span: span_start, value: 'a' })],
    };

    let parser = ParserI {
        parser: Parser {
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
            scratch: RefCell::new(String::from("")),
        },
        pattern,
    };

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_with_greedy() {
    let pattern = "{1,2}?";
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    
    let mut concat = ast::Concat {
        span: span_start,
        asts: vec![Ast::Literal(ast::Literal { span: span_start, value: 'a' })],
    };

    let parser = ParserI {
        parser: Parser {
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
            scratch: RefCell::new(String::from("")),
        },
        pattern,
    };

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_invalid_empty() {
    let pattern = "{1,}";
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    
    let mut concat = ast::Concat {
        span: span_start,
        asts: vec![Ast::Literal(ast::Literal { span: span_start, value: 'a' })],
    };

    let parser = ParserI {
        parser: Parser {
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
            scratch: RefCell::new(String::from("")),
        },
        pattern,
    };

    let _ = parser.parse_counted_repetition(concat);
}

