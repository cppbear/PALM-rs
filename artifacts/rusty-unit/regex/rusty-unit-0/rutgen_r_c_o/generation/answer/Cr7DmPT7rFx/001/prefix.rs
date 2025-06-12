// Answer 0

#[test]
fn test_bump_at_eof() {
    let parser_position = Position { offset: 1, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(parser_position),
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
        pattern: "",
    };
    parser_i.bump();
}

#[test]
fn test_bump_on_empty_pattern() {
    let parser_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(parser_position),
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
        pattern: "",
    };
    parser_i.bump();
}

#[test]
fn test_bump_at_eof_with_single_char() {
    let parser_position = Position { offset: 1, line: 1, column: 2 };
    let parser = Parser {
        pos: Cell::new(parser_position),
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
        pattern: "a",
    };
    parser_i.bump();
}

