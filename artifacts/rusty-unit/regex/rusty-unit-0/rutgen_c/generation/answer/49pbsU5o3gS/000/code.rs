// Answer 0

#[test]
fn test_reset_initializes_position() {
    let parser = Parser::new();
    parser.reset();
    let position = parser.pos.get();
    assert_eq!(position.offset, 0);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 1);
}

#[test]
fn test_reset_initializes_ignore_whitespace() {
    let initial_ignore_whitespace = true;
    let parser = Parser {
        initial_ignore_whitespace,
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    
    parser.reset();
    assert_eq!(parser.ignore_whitespace.get(), initial_ignore_whitespace);
}

#[test]
fn test_reset_clears_comments() {
    let parser = Parser::new();
    parser.comments.borrow_mut().push(Comment {
        span: Span { start: 0, end: 1 },
        comment: "initial comment".to_string(),
    });
    assert!(!parser.comments.borrow().is_empty());

    parser.reset();
    assert!(parser.comments.borrow().is_empty());
}

#[test]
fn test_reset_clears_stack_group() {
    let parser = Parser::new();
    parser.stack_group.borrow_mut().push(GroupState::Group {
        concat: ast::Concat::new(),
        group: ast::Group::new(),
        ignore_whitespace: false,
    });
    assert!(!parser.stack_group.borrow().is_empty());

    parser.reset();
    assert!(parser.stack_group.borrow().is_empty());
}

#[test]
fn test_reset_clears_stack_class() {
    let parser = Parser::new();
    parser.stack_class.borrow_mut().push(ClassState::Open {
        union: ast::ClassSetUnion::new(),
        set: ast::ClassBracketed::new(),
    });
    assert!(!parser.stack_class.borrow().is_empty());

    parser.reset();
    assert!(parser.stack_class.borrow().is_empty());
}

