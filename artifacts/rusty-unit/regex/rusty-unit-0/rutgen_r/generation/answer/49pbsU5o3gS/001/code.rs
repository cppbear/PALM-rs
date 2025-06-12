// Answer 0

#[derive(Default)]
struct MockPosition {
    offset: usize,
    line: usize,
    column: usize,
}

struct MockParser {
    pos: std::cell::RefCell<MockPosition>,
    ignore_whitespace: std::cell::RefCell<bool>,
    comments: std::cell::RefCell<Vec<String>>,
    stack_group: std::cell::RefCell<Vec<String>>,
    stack_class: std::cell::RefCell<Vec<String>>,
    initial_ignore_whitespace: bool,
}

impl MockParser {
    fn new(initial_ignore_whitespace: bool) -> Self {
        MockParser {
            pos: std::cell::RefCell::new(MockPosition::default()),
            ignore_whitespace: std::cell::RefCell::new(initial_ignore_whitespace),
            comments: std::cell::RefCell::new(vec![]),
            stack_group: std::cell::RefCell::new(vec![]),
            stack_class: std::cell::RefCell::new(vec![]),
            initial_ignore_whitespace,
        }
    }

    fn reset(&self) {
        self.pos.borrow_mut().set(MockPosition { offset: 0, line: 1, column: 1 });
        self.ignore_whitespace.replace(self.initial_ignore_whitespace);
        self.comments.borrow_mut().clear();
        self.stack_group.borrow_mut().clear();
        self.stack_class.borrow_mut().clear();
    }
}

#[test]
fn test_reset_function() {
    let parser = MockParser::new(true);

    // Initialize the parser with some data
    parser.comments.borrow_mut().push("Sample comment".to_string());
    parser.stack_group.borrow_mut().push("Group1".to_string());
    parser.stack_class.borrow_mut().push("Class1".to_string());
    
    assert_eq!(parser.comments.borrow().len(), 1);
    assert_eq!(parser.stack_group.borrow().len(), 1);
    assert_eq!(parser.stack_class.borrow().len(), 1);
    assert_eq!(*parser.ignore_whitespace.borrow(), true);

    // Perform the reset
    parser.reset();

    // Check if the parser state has reset to initial values
    assert_eq!(parser.comments.borrow().len(), 0);
    assert_eq!(parser.stack_group.borrow().len(), 0);
    assert_eq!(parser.stack_class.borrow().len(), 0);
    assert_eq!(*parser.ignore_whitespace.borrow(), true);

    let pos = parser.pos.borrow();
    assert_eq!(pos.offset, 0);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

