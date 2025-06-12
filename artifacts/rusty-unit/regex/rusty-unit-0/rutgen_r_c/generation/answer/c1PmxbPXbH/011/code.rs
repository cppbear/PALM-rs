// Answer 0

#[test]
fn test_parse_uncounted_repetition_empty_ast() {
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let mut concat = ast::Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
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
        pattern: "abc*",
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionMissing);
    }
}

#[test]
fn test_parse_uncounted_repetition_flags_ast() {
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let flags_span = Span::new(Position { offset: 3, line: 1, column: 4 }, Position { offset: 4, line: 1, column: 5 });
    let mut concat = ast::Concat { span, asts: vec![Ast::Flags(ast::Flags { span: flags_span })] };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
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
        pattern: "abc*",
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionMissing);
    }
}

