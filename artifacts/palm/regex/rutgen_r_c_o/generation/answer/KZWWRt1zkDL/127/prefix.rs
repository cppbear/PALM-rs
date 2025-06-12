// Answer 0

#[test]
fn test_parse_escape_z() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\z",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_w() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\w",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_U() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\U",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_u() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\u",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_s() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\s",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_d() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\d",
    };

    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_p() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);
    
    let parser = Parser {
        pos: Cell::new(position),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "\\p{scx=Katakana}",
    };

    let _ = parser_i.parse_escape();
}

