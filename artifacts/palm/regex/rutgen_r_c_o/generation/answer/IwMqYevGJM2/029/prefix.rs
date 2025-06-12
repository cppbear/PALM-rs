// Answer 0

#[test]
fn test_parse_unicode_class_single_letter() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
        scratch: RefCell::new(String::from("A")),
    };
    let parser_i = ParserI {
        parser,
        pattern: "\\pA",
    };
    parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_bracketed_notation_with_equal() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
        scratch: RefCell::new(String::from("Greek=123")),
    };
    let parser_i = ParserI {
        parser,
        pattern: "\\p{Greek=123}",
    };
    parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_bracketed_notation_with_colon() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
        scratch: RefCell::new(String::from("Greek:xyz")),
    };
    let parser_i = ParserI {
        parser,
        pattern: "\\p{Greek:xyz}",
    };
    parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_bracketed_notation_with_not_equal() {
    let position = Position { offset: 0, line: 1, column: 1 };
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
        scratch: RefCell::new(String::from("Greek!=abc")),
    };
    let parser_i = ParserI {
        parser,
        pattern: "\\p{Greek!=abc}",
    };
    parser_i.parse_unicode_class();
}

