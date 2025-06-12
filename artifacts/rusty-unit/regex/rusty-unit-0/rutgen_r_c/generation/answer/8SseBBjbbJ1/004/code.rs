// Answer 0

#[test]
fn test_parse_hex_unicode_short_valid() {
    let input = "uFF";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };

    let result = parser.parse_hex();
    assert!(result.is_ok());

    let literal = result.unwrap();
    assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort));
    assert_eq!(literal.c, '\u{FF}');
}

#[test]
fn test_parse_hex_unicode_long_valid() {
    let input = "u{1F600}";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };

    let result = parser.parse_hex();
    assert!(result.is_ok());

    let literal = result.unwrap();
    assert_eq!(literal.kind, ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong));
    assert_eq!(literal.c, '\u{1F600}');
}

#[test]
#[should_panic(expected = "EscapeUnexpectedEof")]
fn test_parse_hex_unexpected_eof() {
    let input = "u";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };

    parser.parse_hex();
}

#[test]
#[should_panic(expected = "EscapeHexInvalid")]
fn test_parse_hex_invalid_digit() {
    let input = "uZZ";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };

    parser.parse_hex();
}

#[test]
#[should_panic(expected = "EscapeHexEmpty")]
fn test_parse_hex_empty_brace() {
    let input = "u{}";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };

    parser.parse_hex();
}

