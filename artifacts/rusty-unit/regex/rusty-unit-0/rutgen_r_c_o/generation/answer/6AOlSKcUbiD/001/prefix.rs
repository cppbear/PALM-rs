// Answer 0

#[test]
fn test_next_capture_index_limit_not_exceeded() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(u32::MAX - 1),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "a(b(c))";
    let parser_i = ParserI::new(&parser, pattern);
    let span = Span { start: 0, end: 1 };
    let _ = parser_i.next_capture_index(span);
}

#[test]
#[should_panic]
fn test_next_capture_index_limit_exceeded() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(u32::MAX),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "a(b(c))";
    let parser_i = ParserI::new(&parser, pattern);
    let span = Span { start: 0, end: 1 };
    let _ = parser_i.next_capture_index(span);
}

#[test]
fn test_next_capture_index_initial_capture_zero() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "(a)";
    let parser_i = ParserI::new(&parser, pattern);
    let span = Span { start: 0, end: 1 };
    let _ = parser_i.next_capture_index(span);
    let _ = parser_i.next_capture_index(span);
}

#[test]
fn test_next_capture_index_multiple_increments() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(2),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "(b(c))";
    let parser_i = ParserI::new(&parser, pattern);
    let span = Span { start: 0, end: 1 };
    let _ = parser_i.next_capture_index(span);
    let _ = parser_i.next_capture_index(span);
    let _ = parser_i.next_capture_index(span);
}

#[test]
fn test_next_capture_index_at_edge() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(u32::MAX - 2),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "(x(y))";
    let parser_i = ParserI::new(&parser, pattern);
    let span = Span { start: 0, end: 1 };
    let _ = parser_i.next_capture_index(span);
}

