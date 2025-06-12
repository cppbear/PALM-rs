// Answer 0

#[test]
fn test_parse_unicode_class_char_not_p() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };
    
    let parser_i = ParserI {
        parser: &parser,
        pattern: "a",
    };
    
    let _ = parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_bump_and_bump_space_false() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "property!=value",
    };

    let _ = parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_char_not_bracket() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "p",
    };

    let _ = parser_i.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_bump_and_bump_space_true() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "property!=value",
    };

    let _ = parser_i.parse_unicode_class();
}

