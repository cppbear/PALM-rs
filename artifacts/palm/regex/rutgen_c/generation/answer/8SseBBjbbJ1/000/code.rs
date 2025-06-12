// Answer 0

#[test]
fn test_parse_hex_x_fixed() {
    let pattern = "\\xFF"; // Hex notation
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
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
        pattern,
    };
    let result = parser.parse_hex();
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::X));
    assert_eq!(literal.c, '\u{FF}');
}

#[test]
fn test_parse_hex_u_brace() {
    let pattern = "\\u{0020}"; // Hex notation with braces
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
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
        pattern,
    };
    let result = parser.parse_hex();
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.kind, ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort));
    assert_eq!(literal.c, ' '); // Unicode codepoint U+0020 is a space
}

#[test]
#[should_panic]
fn test_parse_hex_invalid_digit() {
    let pattern = "\\xZ"; // Invalid hex character
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
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
        pattern,
    };
    let _result = parser.parse_hex();
}

#[test]
fn test_parse_hex_empty_brace() {
    let pattern = "\\u{}"; // Empty braces
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
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
        pattern,
    };
    let result = parser.parse_hex();
    assert!(result.is_err());
}

