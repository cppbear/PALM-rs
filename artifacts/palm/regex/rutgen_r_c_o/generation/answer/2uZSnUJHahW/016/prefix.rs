// Answer 0

#[test]
fn test_parse_hex_brace_valid_hex() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
            scratch: RefCell::new(String::from("007f")),
        },
        pattern: "{007f}",
    };
    let _ = parser.parse_hex_brace(ast::HexLiteralKind::X);
}

#[test]
#[should_panic]
fn test_parse_hex_brace_empty_hex() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
            scratch: RefCell::new(String::new()),
        },
        pattern: "{}",
    };
    let _ = parser.parse_hex_brace(ast::HexLiteralKind::X);
}

#[test]
#[should_panic]
fn test_parse_hex_brace_invalid_hex_digit() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
            scratch: RefCell::new(String::from("0g")),
        },
        pattern: "{0g}",
    };
    let _ = parser.parse_hex_brace(ast::HexLiteralKind::X);
}

#[test]
#[should_panic]
fn test_parse_hex_brace_eof_before_closing_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
            scratch: RefCell::new(String::from("007f")),
        },
        pattern: "{007f",
    };
    let _ = parser.parse_hex_brace(ast::HexLiteralKind::X);
}

