// Answer 0

#[test]
fn test_parse_hex_brace_empty_hex() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
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
    };
    let parser_i = ParserI {
        parser,
        pattern: "{",
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);

    assert!(result.is_err());
}

