// Answer 0

#[test]
fn test_reset() {
    struct Parser {
        pos: Cell<Position>,
        initial_ignore_whitespace: bool,
        ignore_whitespace: Cell<bool>,
        comments: RefCell<Vec<ast::Comment>>,
        stack_group: RefCell<Vec<GroupState>>,
        stack_class: RefCell<Vec<ClassState>>,
    }

    impl Parser {
        fn new() -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
            }
        }

        fn reset(&self) {
            self.pos.set(Position { offset: 0, line: 1, column: 1 });
            self.ignore_whitespace.set(self.initial_ignore_whitespace);
            self.comments.borrow_mut().clear();
            self.stack_group.borrow_mut().clear();
            self.stack_class.borrow_mut().clear();
        }
    }

    // Initialize the parser
    let parser = Parser::new();

    // Verify initial state
    assert_eq!(parser.pos.get(), Position { offset: 0, line: 1, column: 1 });
    assert_eq!(parser.ignore_whitespace.get(), false);
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());

    // Simulate changes to the parser's state
    parser.ignore_whitespace.set(true);
    parser.comments.borrow_mut().push(ast::Comment {
        span: Span { start: 0, end: 0 },
        comment: "test comment".to_string(),
    });
    parser.stack_group.borrow_mut().push(GroupState::Group {
        concat: ast::Concat::default(),
        group: ast::Group::default(),
        ignore_whitespace: false,
    });

    // Call reset
    parser.reset();

    // Check state after reset
    assert_eq!(parser.pos.get(), Position { offset: 0, line: 1, column: 1 });
    assert_eq!(parser.ignore_whitespace.get(), false); // should reset to initial
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
}

#[test]
fn test_reset_with_initial_whitespace() {
    struct Parser {
        pos: Cell<Position>,
        initial_ignore_whitespace: bool,
        ignore_whitespace: Cell<bool>,
        comments: RefCell<Vec<ast::Comment>>,
        stack_group: RefCell<Vec<GroupState>>,
        stack_class: RefCell<Vec<ClassState>>,
    }

    impl Parser {
        fn new(initial_ignore_whitespace: bool) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                initial_ignore_whitespace,
                ignore_whitespace: Cell::new(initial_ignore_whitespace),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
            }
        }

        fn reset(&self) {
            self.pos.set(Position { offset: 0, line: 1, column: 1 });
            self.ignore_whitespace.set(self.initial_ignore_whitespace);
            self.comments.borrow_mut().clear();
            self.stack_group.borrow_mut().clear();
            self.stack_class.borrow_mut().clear();
        }
    }

    let parser = Parser::new(true);

    // Verify initial state
    assert_eq!(parser.ignore_whitespace.get(), true);

    // Call reset
    parser.reset();

    // After reset, should reflect initial_whitespace
    assert_eq!(parser.ignore_whitespace.get(), true);
}

