// Answer 0

#[test]
fn test_parse_empty_regex() {
    struct DummyParser;
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { line: 0, column: 0 }),
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
            }
        }
    }

    let parser = ParserI {
        parser: DummyParser,
        pattern: "",
    };
    
    let result = parser.parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ast::Empty(Span::default()));
}

#[test]
fn test_parse_simple_literal() {
    struct DummyParser;
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { line: 0, column: 0 }),
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
            }
        }
    }

    let parser = ParserI {
        parser: DummyParser,
        pattern: "a",
    };

    let result = parser.parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ast::Literal(ast::Literal::from_char('a')));
}

#[test]
fn test_parse_nested_groups() {
    struct DummyParser;
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { line: 0, column: 0 }),
                capture_index: Cell::new(0),
                nest_limit: 2,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser = ParserI {
        parser: DummyParser,
        pattern: "(ab(cd|ef)gh)",
    };

    let result = parser.parse();
    assert!(result.is_ok());
    // Further checks on the resulting AST could be added
}

#[test]
#[should_panic]
fn test_parse_exceeds_nest_limit() {
    struct DummyParser;
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { line: 0, column: 0 }),
                capture_index: Cell::new(0),
                nest_limit: 1,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser = ParserI {
        parser: DummyParser,
        pattern: "((a)b)",
    };

    parser.parse();
}

#[test]
fn test_parse_single_dot() {
    struct DummyParser;
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { line: 0, column: 0 }),
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
            }
        }
    }

    let parser = ParserI {
        parser: DummyParser,
        pattern: ".",
    };

    let result = parser.parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ast::Dot(Span::default()));
}

