// Answer 0

#[test]
fn test_parse_counted_repetition_valid() {
    let example_pattern = "{3,5}";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let concat = ast::Concat {
        span: Span { start: start_pos, end: start_pos },
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, start_pos) })],
    };
    
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    
    let parser_i = ParserI {
        parser: &parser,
        pattern: example_pattern,
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_valid_unbounded() {
    let example_pattern = "{3,}";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let concat = ast::Concat {
        span: Span { start: start_pos, end: start_pos },
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, start_pos) })],
    };

    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: example_pattern,
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_single() {
    let example_pattern = "{3}";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let concat = ast::Concat {
        span: Span { start: start_pos, end: start_pos },
        asts: vec![Ast::Literal(ast::Literal { span: Span::new(start_pos, start_pos) })],
    };

    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: example_pattern,
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

