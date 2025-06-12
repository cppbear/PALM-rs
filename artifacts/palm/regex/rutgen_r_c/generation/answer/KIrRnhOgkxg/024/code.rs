// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid() {
    let pattern = "[[:alnum:]]";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: &Parser {
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
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());

    let expected_span = Span::new(position, Position { offset: 13, line: 1, column: 14 });
    let expected_kind = ast::ClassAsciiKind::from_name("alnum").unwrap();

    if let Some(class_ascii) = result {
        assert_eq!(class_ascii.span, expected_span);
        assert_eq!(class_ascii.kind, expected_kind);
        assert!(!class_ascii.negated); // As it is not negated
    }
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    let pattern = "[[:^digit:]]";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: &Parser {
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
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());

    let expected_span = Span::new(position, Position { offset: 13, line: 1, column: 14 });
    let expected_kind = ast::ClassAsciiKind::from_name("digit").unwrap();

    if let Some(class_ascii) = result {
        assert_eq!(class_ascii.span, expected_span);
        assert_eq!(class_ascii.kind, expected_kind);
        assert!(class_ascii.negated); // As it is negated
    }
}

#[test]
fn test_maybe_parse_ascii_class_invalid() {
    let pattern = "[[:invalid:]]";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: &Parser {
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
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_missing_closing_bracket() {
    let pattern = "[[:lower:";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: &Parser {
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
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

