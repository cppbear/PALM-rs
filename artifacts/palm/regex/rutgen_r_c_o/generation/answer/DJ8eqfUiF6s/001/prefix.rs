// Answer 0

#[test]
fn test_span_char_normal_case() {
    let pattern = "abc";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, pattern);
    let span = parser_i.span_char();
}

#[test]
fn test_span_char_newline_case() {
    let pattern = "hello\nworld";
    let pos = Position { offset: 5, line: 1, column: 6 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, pattern);
    let span = parser_i.span_char();
}

#[test]
fn test_span_char_edge_case_offset_max() {
    let pattern = "a";
    let pos = Position { offset: usize::MAX - 1, line: usize::MAX, column: usize::MAX - 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, pattern);
    let span = parser_i.span_char();
}

#[test]
fn test_span_char_edge_case_line_increment() {
    let pattern = "ab\ncd";
    let pos = Position { offset: 2, line: 1, column: 3 }; 
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, pattern);
    let span = parser_i.span_char();
}

