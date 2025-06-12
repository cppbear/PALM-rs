// Answer 0

#[test]
fn test_parse_group_non_capturing_flags() {
    let pattern = "(?i)";
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span { start: position, end: position };
    
    let flags = ast::Flags {
        span: span_start,
        items: vec![
            ast::FlagsItem {
                span: span_start,
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreCase),
            },
        ],
    };
    
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
        pattern,
    };

    parser.bump_if("(?") = true;
    assert_eq!(parser.bump_if("?P<"), false);
    assert_eq!(parser.bump_if("?"), true);
    assert_eq!(parser.is_eof(), false);
    parser.parse_flags() = Ok(flags);
    let char_end = parser.char();
    assert_eq!(char_end, ')');

    let result = parser.parse_group();
}

