// Answer 0

#[test]
fn test_next_capture_index_zero() {
    let span = Span { start: 0, end: 1 };
    let parser = Parser {
        pos: Cell::new(0),
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
    };
    let parser_i = ParserI::new(&parser, "test");
    let _ = parser_i.next_capture_index(span);
}

#[test]
fn test_next_capture_index_maximum() {
    let span = Span { start: 0, end: 1 };
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(u32::MAX - 1),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test");
    let _ = parser_i.next_capture_index(span);
}

#[test]
#[should_panic]
fn test_next_capture_index_overflow() {
    let span = Span { start: 0, end: 1 };
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(u32::MAX),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test");
    let _ = parser_i.next_capture_index(span);
}

