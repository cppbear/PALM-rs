// Answer 0

#[test]
fn test_span_char_with_valid_position() {
    struct MockParser {
        postion: Position,
        current_char: char,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.position),
                capture_index: Cell::new(0),
                nest_limit: 100,
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

    let initial_position = Position { offset: 0, line: 1, column: 1 };
    let mock_parser = MockParser {
        position: initial_position,
        current_char: 'a',
    };

    let parser_instance = ParserI::new(mock_parser, "a");
    let span = parser_instance.span_char();

    assert_eq!(span.start.offset, 0);
    assert_eq!(span.end.offset, 1);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.column, 2);
}

#[test]
#[should_panic(expected = "Some panic message related to checked_add")]
fn test_span_char_with_panic_on_offset() {
    struct MockParser {
        position: Position,
        current_char: char,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.position),
                capture_index: Cell::new(0),
                nest_limit: 100,
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

    let initial_position = Position { offset: usize::MAX, line: 1, column: 1 };
    let mock_parser = MockParser {
        position: initial_position,
        current_char: 'a',
    };

    let parser_instance = ParserI::new(mock_parser, "a");
    parser_instance.span_char();
}

#[test]
#[should_panic(expected = "Some panic message related to checked_add")]
fn test_span_char_with_panic_on_column() {
    struct MockParser {
        position: Position,
        current_char: char,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.position),
                capture_index: Cell::new(0),
                nest_limit: 100,
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

    let initial_position = Position { offset: 0, line: 1, column: usize::MAX };
    let mock_parser = MockParser {
        position: initial_position,
        current_char: 'a',
    };

    let parser_instance = ParserI::new(mock_parser, "a");
    parser_instance.span_char();
}

