// Answer 0

#[test]
fn test_char_valid_position() {
    struct MockParser {
        pos: Position,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
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

    let pattern = "abc";
    let parser = MockParser { pos: Position { offset: 0 } };
    let parser_i = ParserI::new(parser, pattern);
    assert_eq!(parser_i.char(), 'a');

    let parser = MockParser { pos: Position { offset: 1 } };
    let parser_i = ParserI::new(parser, pattern);
    assert_eq!(parser_i.char(), 'b');

    let parser = MockParser { pos: Position { offset: 2 } };
    let parser_i = ParserI::new(parser, pattern);
    assert_eq!(parser_i.char(), 'c');
}

#[test]
#[should_panic(expected = "expected char at offset 3")]
fn test_char_invalid_position() {
    struct MockParser {
        pos: Position,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
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

    let pattern = "abc";
    let parser = MockParser { pos: Position { offset: 3 } };
    let parser_i = ParserI::new(parser, pattern);
    parser_i.char();
}

