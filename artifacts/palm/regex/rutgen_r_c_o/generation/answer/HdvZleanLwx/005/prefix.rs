// Answer 0

#[test]
fn test_parse_octal_with_minimum_digit() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start, end);
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "0" };
    let literal = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_with_three_digits() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(start, end);
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "777" };
    let literal = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_with_invalid_digit() {
    let start = Position { offset: 0, line: 1, column: 1 };
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "8" };
    let literal = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_with_empty_string() {
    let start = Position { offset: 0, line: 1, column: 1 };
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "" };
    let literal = parser_i.parse_octal();
}

