// Answer 0

#[test]
fn test_parse_decimal_empty_input() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    
    let result = parser_i.parse_decimal();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::DecimalEmpty);
    }
}

#[test]
fn test_parse_decimal_invalid_characters() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut scratch = String::from("12a34");
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(scratch),
    };
    let parser_i = ParserI { parser, pattern: "" };
    
    let result = parser_i.parse_decimal();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::DecimalInvalid);
    }
}

#[test]
fn test_parse_decimal_leading_whitespace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut scratch = String::from("   123    ");
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(scratch),
    };
    let parser_i = ParserI { parser, pattern: "" };

    let result = parser_i.parse_decimal();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123);
}

#[test]
fn test_parse_decimal_trailing_whitespace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut scratch = String::from("456   ");
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(scratch),
    };
    let parser_i = ParserI { parser, pattern: "" };

    let result = parser_i.parse_decimal();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 456);
}

