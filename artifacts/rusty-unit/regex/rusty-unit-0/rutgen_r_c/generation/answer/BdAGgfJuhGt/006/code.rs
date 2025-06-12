// Answer 0

#[test]
fn test_bump_space_with_comments_and_whitespace() {
    struct TestParser {
        ignore_whitespace: bool,
        pattern: String,
        pos: Position,
        comments: RefCell<Vec<ast::Comment>>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: true,
                initial_ignore_whitespace: true,
                ignore_whitespace: Cell::new(self.ignore_whitespace),
                comments: RefCell::new(self.comments.borrow().clone()),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let initial_position = Position { offset: 0, line: 1, column: 1 };
    let mut comments = vec![];

    // Create a TestParser instance with ignore_whitespace set to true
    let mut parser = TestParser {
        ignore_whitespace: true,
        pattern: "   # this is a comment\n   non-whitespace".to_string(),
        pos: initial_position,
        comments: RefCell::new(comments),
    };

    let parser_instance = ParserI::new(&mut parser, &parser.pattern);
    
    // Call bump_space method
    parser_instance.bump_space();

    // Assert that the correct comment is captured
    assert_eq!(parser.comments.borrow().len(), 1);
    assert_eq!(parser.comments.borrow()[0].comment, "this is a comment");
    assert_eq!(parser.comments.borrow()[0].span.start.offset, 3);
    assert_eq!(parser.comments.borrow()[0].span.end.offset, 24);
    assert_eq!(parser_instance.offset(), 27); // The position should be after the comments and whitespace
}

#[test]
fn test_bump_space_no_ignore_whitespace() {
    struct TestParser {
        ignore_whitespace: bool,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: true,
                initial_ignore_whitespace: true,
                ignore_whitespace: Cell::new(self.ignore_whitespace),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    // Create a TestParser instance with ignore_whitespace set to false
    let parser = TestParser {
        ignore_whitespace: false,
        pattern: "   non-whitespace".to_string(),
        pos: Position { offset: 0, line: 1, column: 1 },
    };

    let parser_instance = ParserI::new(parser, &parser.pattern);
    
    // Call bump_space method
    parser_instance.bump_space();

    // Assert there are no comments captured and the offset stays the same
    assert!(parser_instance.parser().comments.borrow().is_empty());
    assert_eq!(parser_instance.offset(), 0);
}

