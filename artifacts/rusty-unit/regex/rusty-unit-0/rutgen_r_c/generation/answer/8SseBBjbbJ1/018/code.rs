// Answer 0

#[test]
fn test_parse_hex_with_x_invalid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 32,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from("")),
        },
        pattern: r"\x",
    };

    let result = parser.parse_hex();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

#[test]
fn test_parse_hex_with_u_invalid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 32,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from("")),
        },
        pattern: r"\u",
    };

    let result = parser.parse_hex();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
} 

#[test]
fn test_parse_hex_with_x_empty() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 32,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::from("")),
        },
        pattern: r"\x{}",
    };

    let result = parser.parse_hex();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeHexEmpty);
    }
}

