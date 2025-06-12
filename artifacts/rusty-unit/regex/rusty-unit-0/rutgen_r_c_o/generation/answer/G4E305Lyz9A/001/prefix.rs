// Answer 0

#[test]
fn test_finish_with_valid_state() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: parser,
        pattern: "a(b(c(d)e)f)g",
    };

    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth: 3,
    };

    let _ = nest_limiter.finish();
}

#[test]
fn test_finish_with_zero_depth() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(10),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: parser,
        pattern: "",
    };

    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth: 0,
    };

    let _ = nest_limiter.finish();
}

#[test]
fn test_finish_with_high_capture_index() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(100),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: parser,
        pattern: ".*",
    };

    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth: 5,
    };

    let _ = nest_limiter.finish();
}

#[test]
fn test_finish_with_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(1),
        nest_limit: 3,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: parser,
        pattern: "",
    };

    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth: 1,
    };

    let _ = nest_limiter.finish();
}

#[test]
fn test_finish_with_high_nest_limit() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: parser,
        pattern: "(a(b(c(d))))",
    };

    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth: 4,
    };

    let _ = nest_limiter.finish();
}

