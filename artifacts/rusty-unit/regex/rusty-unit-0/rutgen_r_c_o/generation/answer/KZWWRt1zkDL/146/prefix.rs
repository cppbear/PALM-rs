// Answer 0

#[test]
fn test_parse_escape_unicode_class() {
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
        scratch: RefCell::new(String::new()),
    };

    let pattern = "\\p{scx=Latin}";
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_escape();
} 

#[test]
fn test_parse_escape_negated_unicode_class() {
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
        scratch: RefCell::new(String::new()),
    };

    let pattern = "\\P{scx=Latin}";
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_escape();
} 

#[test]
fn test_parse_escape_unicode_class_empty() {
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
        scratch: RefCell::new(String::new()),
    };

    let pattern = "\\p{}";
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_escape();
} 

#[test]
fn test_parse_escape_unicode_class_invalid_name() {
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
        scratch: RefCell::new(String::new()),
    };

    let pattern = "\\p{invalid-name}";
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_escape();
} 

#[test]
fn test_parse_escape_multiline_pattern() {
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
        scratch: RefCell::new(String::new()),
    };

    let pattern = "\\p{scx=Greek}\\p{scx=Latin}";
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_escape();
} 

