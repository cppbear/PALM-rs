// Answer 0

#[test]
fn test_offset() {
    struct MockParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: self.pos.clone(),
                capture_index: Cell::new(0),
                nest_limit: 0,
                octal: true,
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

    let position = Position { offset: 10, line: 1, column: 1 };
    let parser = MockParser { pos: Cell::new(position) };
    let parser_instance = ParserI::new(parser, "abc");

    assert_eq!(parser_instance.offset(), 10);

    let position_zero = Position { offset: 0, line: 1, column: 1 };
    let parser_zero = MockParser { pos: Cell::new(position_zero) };
    let parser_instance_zero = ParserI::new(parser_zero, "abc");

    assert_eq!(parser_instance_zero.offset(), 0);
}

