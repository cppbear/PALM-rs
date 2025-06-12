// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI { parser: &parser, pattern: "a-z" };
    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_valid_literals() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 10, line: 1, column: 11 }),
        capture_index: Cell::new(2),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI { parser: &parser, pattern: "b-d" };
    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_spaces() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 20, line: 1, column: 21 }),
        capture_index: Cell::new(1),
        nest_limit: 7,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI { parser: &parser, pattern: " e - f " };
    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_characters() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 30, line: 1, column: 31 }),
        capture_index: Cell::new(3),
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

    let parser_i = ParserI { parser: &parser, pattern: "x - y" };
    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_same_characters() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 40, line: 1, column: 41 }),
        capture_index: Cell::new(4),
        nest_limit: 2,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI { parser: &parser, pattern: "a - a" };
    parser_i.parse_set_class_range();
}

