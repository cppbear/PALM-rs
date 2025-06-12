// Answer 0

#[test]
fn test_parse_decimal_eof_true() {
    let parser = Parser {
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "   " };
    let result = parser_i.parse_decimal();
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_parse_decimal_eof_false_no_digits() {
    let parser = Parser {
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
        scratch: RefCell::new(String::from("   ")),
    };
    let parser_i = ParserI { parser: &parser, pattern: "   " };
    let result = parser_i.parse_decimal();
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_parse_decimal_eof_false_digits_invalid() {
    let parser = Parser {
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
        scratch: RefCell::new(String::from("abc")),
    };
    let parser_i = ParserI { parser: &parser, pattern: "abc" };
    let result = parser_i.parse_decimal();
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_parse_decimal_valid() {
    let mut scratch_buffer = RefCell::new(String::new());
    scratch_buffer.borrow_mut().push_str("1234");
    let parser = Parser {
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
        scratch: scratch_buffer,
    };
    let parser_i = ParserI { parser: &parser, pattern: "   1234   " };
    let result = parser_i.parse_decimal();
    assert_eq!(result, Ok(1234));
}

#[test]
fn test_parse_decimal_valid_multiple_spaces() {
    let mut scratch_buffer = RefCell::new(String::new());
    scratch_buffer.borrow_mut().push_str("5678");
    let parser = Parser {
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
        scratch: scratch_buffer,
    };
    let parser_i = ParserI { parser: &parser, pattern: "     5678    " };
    let result = parser_i.parse_decimal();
    assert_eq!(result, Ok(5678));
}

