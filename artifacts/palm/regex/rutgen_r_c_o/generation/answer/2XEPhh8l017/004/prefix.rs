// Answer 0

#[test]
fn test_parse_hex_digits_eof_2_digits() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\x" };
    let kind = HexLiteralKind::X;

    let _ = parser_i.parse_hex_digits(kind);
}

#[test]
fn test_parse_hex_digits_eof_4_digits() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\u" };
    let kind = HexLiteralKind::UnicodeShort;

    let _ = parser_i.parse_hex_digits(kind);
}

#[test]
fn test_parse_hex_digits_eof_8_digits() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\U" };
    let kind = HexLiteralKind::UnicodeLong;

    let _ = parser_i.parse_hex_digits(kind);
}

