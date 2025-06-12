// Answer 0

#[test]
fn test_reset() {
    struct MockPosition {
        offset: usize,
        line: usize,
        column: usize,
    }

    struct MockParser {
        pos: MockPosition,
        initial_ignore_whitespace: bool,
        ignore_whitespace: std::cell::RefCell<bool>,
        comments: std::cell::RefCell<Vec<String>>,
        stack_group: std::cell::RefCell<Vec<u32>>,
        stack_class: std::cell::RefCell<Vec<u32>>,
    }

    impl MockParser {
        fn new(initial_ignore_whitespace: bool) -> Self {
            Self {
                pos: MockPosition { offset: 0, line: 1, column: 1 },
                initial_ignore_whitespace,
                ignore_whitespace: std::cell::RefCell::new(initial_ignore_whitespace),
                comments: std::cell::RefCell::new(vec![]),
                stack_group: std::cell::RefCell::new(vec![]),
                stack_class: std::cell::RefCell::new(vec![]),
            }
        }

        fn reset(&self) {
            self.pos = MockPosition { offset: 0, line: 1, column: 1 };
            *self.ignore_whitespace.borrow_mut() = self.initial_ignore_whitespace;
            self.comments.borrow_mut().clear();
            self.stack_group.borrow_mut().clear();
            self.stack_class.borrow_mut().clear();
        }
    }

    let parser = MockParser::new(true);
    parser.comments.borrow_mut().push("Comment".to_string());
    parser.stack_group.borrow_mut().push(1);
    parser.stack_class.borrow_mut().push(2);
    *parser.ignore_whitespace.borrow_mut() = false;

    parser.reset();

    assert_eq!(parser.pos.offset, 0);
    assert_eq!(parser.pos.line, 1);
    assert_eq!(parser.pos.column, 1);
    assert_eq!(*parser.ignore_whitespace.borrow(), true);
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
}

