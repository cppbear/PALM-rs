// Answer 0

#[test]
fn test_parse_escape_invalid_octal() {
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);
    
    struct MockParser {
        octal: bool,
        position: Position,
        pattern: String,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.position), capture_index: Cell::new(0), nest_limit: 5, octal: self.octal, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }
        }
    }

    let mock_pattern = "8";
    let parser = MockParser { octal: false, position: pos_start, pattern: mock_pattern.to_string() };

    let parser_i = ParserI { parser: &parser, pattern: &parser.pattern };

    let result = parser_i.parse_escape();

    match result {
        Err(error) => {
            assert_eq!(error.kind, ErrorKind::UnsupportedBackreference);
            assert_eq!(error.span, Span::new(pos_start, pos_end));
        }
        _ => panic!("Expected an error, but got a result."),
    }
}

#[test]
fn test_parse_escape_invalid_digit() {
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);
    
    struct MockParser {
        octal: bool,
        position: Position,
        pattern: String,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.position), capture_index: Cell::new(0), nest_limit: 5, octal: self.octal, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }
        }
    }

    let mock_pattern = "9";
    let parser = MockParser { octal: false, position: pos_start, pattern: mock_pattern.to_string() };

    let parser_i = ParserI { parser: &parser, pattern: &parser.pattern };

    let result = parser_i.parse_escape();

    match result {
        Err(error) => {
            assert_eq!(error.kind, ErrorKind::UnsupportedBackreference);
            assert_eq!(error.span, Span::new(pos_start, pos_end));
        }
        _ => panic!("Expected an error, but got a result."),
    }
}

