// Answer 0

#[test]
fn test_is_lookaround_prefix_true_case() {
    struct TestParser {
        position: usize,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 0,
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

    impl TestParser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            let sub_pattern = &self.pattern[self.position..];
            if sub_pattern.starts_with(prefix) {
                self.position += prefix.chars().count();
                true
            } else {
                false
            }
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
            || self.bump_if("?!")
            || self.bump_if("?<=")
            || self.bump_if("?<!")
        }
    }

    let mut parser = TestParser {
        position: 0,
        pattern: String::from("(?<=abc)xyz"),
    };

    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_false_case() {
    struct TestParser {
        position: usize,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 0,
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

    impl TestParser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            let sub_pattern = &self.pattern[self.position..];
            if sub_pattern.starts_with(prefix) {
                self.position += prefix.chars().count();
                true
            } else {
                false
            }
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
            || self.bump_if("?!")
            || self.bump_if("?<=")
            || self.bump_if("?<!")
        }
    }

    let mut parser = TestParser {
        position: 0,
        pattern: String::from("(abc)xyz"),
    };

    assert!(!parser.is_lookaround_prefix());
}

