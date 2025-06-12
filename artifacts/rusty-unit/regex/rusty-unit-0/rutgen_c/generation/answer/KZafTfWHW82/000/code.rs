// Answer 0

#[test]
fn test_parse_empty_pattern() {
    struct DummyParser;

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
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
            }
        }
    }

    let parser_i = ParserI {
        parser: DummyParser,
        pattern: "",
    };

    let result = parser_i.parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ast::Empty(Span::default()));
}

#[test]
fn test_parse_single_character() {
    struct DummyParser;

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
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
            }
        }
    }

    let parser_i = ParserI {
        parser: DummyParser,
        pattern: "a",
    };

    let result = parser_i.parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ast::Literal(ast::Literal::new('a', Span::default())));
}

#[test]
fn test_parse_alternation() {
    struct DummyParser;

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
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
            }
        }
    }

    let parser_i = ParserI {
        parser: DummyParser,
        pattern: "a|b",
    };

    let result = parser_i.parse();
    assert!(result.is_ok());
    if let Ast::Alternation(ref alternation) = result.unwrap() {
        assert_eq!(alternation.asts.len(), 2);
    } else {
        panic!("Expected an Alternation AST");
    }
}

