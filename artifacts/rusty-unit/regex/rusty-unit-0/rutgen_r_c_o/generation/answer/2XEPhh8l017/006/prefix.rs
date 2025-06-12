// Answer 0

#[test]
fn test_parse_hex_digits_invalid_characters() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 2, line: 1, column: 3 };
    let pattern = "\\xzz"; // Invalid hex representation for `\x`

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_position),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(pattern)),
        },
        pattern,
    };

    let kind = ast::HexLiteralKind::X;
    let _result = parser.parse_hex_digits(kind);
}

#[test]
fn test_parse_hex_digits_invalid_length() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 1, line: 1, column: 2 };
    let pattern = "\\x1"; // Incomplete hex representation

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_position),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(pattern)),
        },
        pattern,
    };

    let kind = ast::HexLiteralKind::X;
    let _result = parser.parse_hex_digits(kind);
}

