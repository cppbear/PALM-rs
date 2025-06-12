// Answer 0

#[test]
fn test_increment_depth_within_limit() {
    struct MockParser {
        nest_limit: u32,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
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

    let span = Span { start: 0, end: 1 };
    let parser = MockParser { nest_limit: 1 };
    let mut limiter = NestLimiter::new(&ParserI::new(parser, "pattern"));

    limiter.depth = 1; // Set depth to the limit
    let result = limiter.increment_depth(&span);
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 2); // Should have incremented the depth
}

#[test]
#[should_panic]
fn test_increment_depth_exceeds_u32_max() {
    struct MockParser {
        nest_limit: u32,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
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

    let span = Span { start: 0, end: 1 };
    let parser = MockParser { nest_limit: 1 };
    let mut limiter = NestLimiter::new(&ParserI::new(parser, "pattern"));

    limiter.depth = u32::MAX; // Set depth to max
    limiter.increment_depth(&span).unwrap(); // This should trigger a panic
} 

#[test]
fn test_increment_depth_reaches_limit() {
    struct MockParser {
        nest_limit: u32,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
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

    let span = Span { start: 0, end: 1 };
    let parser = MockParser { nest_limit: 1 };
    let mut limiter = NestLimiter::new(&ParserI::new(parser, "pattern"));

    limiter.depth = 0; // Set initial depth
    let result = limiter.increment_depth(&span); // Should increment and be within limit
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 1); // Depth should be incremented
} 

#[test]
fn test_increment_depth_at_limit() {
    struct MockParser {
        nest_limit: u32,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
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

    let span = Span { start: 0, end: 1 };
    let parser = MockParser { nest_limit: 1 };
    let mut limiter = NestLimiter::new(&ParserI::new(parser, "pattern"));

    limiter.depth = 1; // Set depth to the limit
    let result = limiter.increment_depth(&span); // Should fail as it exceeds limit
    assert!(result.is_err());
    assert_eq!(limiter.depth, 1); // Depth should not be incremented
} 

