// Answer 0

#[test]
fn test_is_lookaround_prefix_with_positive_case_equals() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 100,
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
    
    let parser = ParserI::new(MockParser, "?=Hello");
    
    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_with_positive_case_not_equals() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 100,
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

    let parser = ParserI::new(MockParser, "?!Hello");
    
    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_with_positive_case_before_equals() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 100,
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
  
    let parser = ParserI::new(MockParser, "?<=Hello");
    
    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_with_positive_case_not_before_equals() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 100,
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
    
    let parser = ParserI::new(MockParser, "?<!Hello");
    
    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_with_no_lookaround_prefix() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 100,
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
  
    let parser = ParserI::new(MockParser, "Hello");
    
    assert!(!parser.is_lookaround_prefix());
}

