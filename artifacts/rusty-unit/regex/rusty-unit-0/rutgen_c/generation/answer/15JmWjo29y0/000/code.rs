// Answer 0

#[test]
fn test_parser_pos() {
    struct MockParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unsafe { mem::transmute(self) }
        }
    }

    let initial_position = Position {
        offset: 5,
        line: 2,
        column: 3,
    };

    let parser = Parser {
        pos: Cell::new(initial_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(MockParser { pos: Cell::new(initial_position) }, "a*b");

    let position = parser_i.pos();
    
    assert_eq!(position.offset, initial_position.offset);
    assert_eq!(position.line, initial_position.line);
    assert_eq!(position.column, initial_position.column);
}

