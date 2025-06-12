// Answer 0

#[test]
fn test_parse_hex_digits_invalid_empty() {
    let kind = ast::HexLiteralKind::X; // Testing for 2-digit hex, expecting an empty sequence
    let position = ast::Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { 
        parser: Parser { 
            pos: Cell::new(position), 
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
        }, 
        pattern: "\\x"
    };
    let result = parser.parse_hex_digits(kind);
}

#[test]
fn test_parse_hex_digits_invalid_non_hex() {
    let kind = ast::HexLiteralKind::UnicodeShort; // Testing for 4-digit hex
    let position = ast::Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { 
        parser: Parser { 
            pos: Cell::new(position), 
            capture_index: Cell::new(0), 
            nest_limit: 10, 
            octal: true, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new("gxyz".to_string()), // Not hex chars
        },
        pattern: "\\u0047"
    };
    let result = parser.parse_hex_digits(kind);
}

#[test]
fn test_parse_hex_digits_invalid_out_of_bounds() {
    let kind = ast::HexLiteralKind::UnicodeLong; // Testing for 8-digit hex
    let position = ast::Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { 
        parser: Parser { 
            pos: Cell::new(position), 
            capture_index: Cell::new(0), 
            nest_limit: 10, 
            octal: true, 
            initial_ignore_whitespace: false, 
            ignore_whitespace: Cell::new(false), 
            comments: RefCell::new(vec![]), 
            stack_group: RefCell::new(vec![]), 
            stack_class: RefCell::new(vec![]), 
            capture_names: RefCell::new(vec![]), 
            scratch: RefCell::new("F0F0F0F0".to_string()), // Maximum valid 
        },
        pattern: "\\U00000000"
    };
    let result = parser.parse_hex_digits(kind);
}

