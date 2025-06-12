// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_opening() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 2 };
    let span_end = Position { offset: 1, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    let concat = ast::Concat {
        span: span,
        asts: vec![Ast::Literal(ast::Literal { span })],
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
            scratch: RefCell::new("".to_string()),
        },
        pattern: "{2,3}",
    };
    
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 3 };
    let span_end = Position { offset: 0, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    let concat = ast::Concat {
        span: span,
        asts: vec![],
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
            scratch: RefCell::new("".to_string()),
        },
        pattern: "{2,3}",
    };

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_eof_reached() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 2 };
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    let concat = ast::Concat {
        span: span,
        asts: vec![Ast::Literal(ast::Literal { span })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 3, line: 1, column: 4 }),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new("".to_string()),
        },
        pattern: "{2,3}",
    };

    let _ = parser.parse_counted_repetition(concat);
}

