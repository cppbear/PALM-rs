// Answer 0

#[test]
fn test_decrement_depth_success() {
    let mut parser_i = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a(bc)d",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 5;
    nest_limiter.decrement_depth();
}

#[test]
#[should_panic]
fn test_decrement_depth_panics() {
    let mut parser_i = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a(bc)d",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 1;
    nest_limiter.decrement_depth();
}

#[test]
fn test_decrement_depth_reaches_zero() {
    let mut parser_i = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a(bc)d",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 2;
    nest_limiter.decrement_depth();
    nest_limiter.decrement_depth();
}

