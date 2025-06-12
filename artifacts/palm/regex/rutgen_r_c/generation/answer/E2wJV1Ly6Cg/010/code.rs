// Answer 0

#[test]
fn test_peek_space_with_whitespace_insensitive_mode() {
    struct TestParser {
        pos: Cell<ast::Position>,
        capture_index: Cell<u32>,
        nest_limit: u32,
        octal: bool,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: self.pos.clone(),
                capture_index: self.capture_index.clone(),
                nest_limit: self.nest_limit,
                octal: self.octal,
                initial_ignore_whitespace: false,
                ignore_whitespace: self.ignore_whitespace.clone(),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let pattern = "a b c#comment\n d e";
    let parser = TestParser {
        pos: Cell::new(ast::Position { offset: 0 }), // initial position
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        ignore_whitespace: Cell::new(true), // whitespace insensitive mode
        pattern: pattern.to_string(),
    };

    let parser_instance = ParserI::new(parser, &parser.pattern);
    
    // Invoke peek_space
    let result = parser_instance.peek_space();

    // Expected outcome: the first non-whitespace character after ignoring
    // the initial characters and comments is 'd'.
    assert_eq!(result, Some('d'));
}

#[test]
fn test_peek_space_is_eof_false() {
    struct TestParser {
        pos: Cell<ast::Position>,
        capture_index: Cell<u32>,
        nest_limit: u32,
        octal: bool,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: self.pos.clone(),
                capture_index: self.capture_index.clone(),
                nest_limit: self.nest_limit,
                octal: self.octal,
                initial_ignore_whitespace: false,
                ignore_whitespace: self.ignore_whitespace.clone(),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let pattern = " a  \n #some comment\n\n b";
    let parser = TestParser {
        pos: Cell::new(ast::Position { offset: 5 }), // position just before whitespace
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        ignore_whitespace: Cell::new(true),
        pattern: pattern.to_string(),
    };

    let parser_instance = ParserI::new(parser, &parser.pattern);
    
    // Invoke peek_space
    let result = parser_instance.peek_space();

    // Expected: the first non-whitespace character after spaces is 'b'
    assert_eq!(result, Some('b'));
}

#[test]
#[should_panic(expected = "invalid index")]
fn test_peek_space_edge_case_invalid_index() {
    struct TestParser {
        pos: Cell<ast::Position>,
        capture_index: Cell<u32>,
        nest_limit: u32,
        octal: bool,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: self.pos.clone(),
                capture_index: self.capture_index.clone(),
                nest_limit: self.nest_limit,
                octal: self.octal,
                initial_ignore_whitespace: false,
                ignore_whitespace: self.ignore_whitespace.clone(),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let pattern = ""; // empty pattern should trigger panics
    let parser = TestParser {
        pos: Cell::new(ast::Position { offset: 0 }), // initial position
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        ignore_whitespace: Cell::new(true),
        pattern: pattern.to_string(),
    };

    let parser_instance = ParserI::new(parser, &parser.pattern);
    
    // Invoke peek_space
    parser_instance.peek_space(); // should panic
}

