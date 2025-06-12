// Answer 0

#[test]
fn test_parse_unicode_class_negated_with_not_equal() {
    let pattern = r"\p{property!=value}";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 100,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::from("")),
    };

    let parser_instance = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_instance.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_positioned_at_p() {
    let pattern = r"\p";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 100,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::from("")),
    };

    let parser_instance = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_instance.parse_unicode_class();
}

#[test]
#[should_panic]
fn test_parse_unicode_class_empty_name() {
    let pattern = r"\p{}";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 100,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::from("")),
    };

    let parser_instance = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_instance.parse_unicode_class();
}

