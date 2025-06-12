// Answer 0

#[test]
fn test_parse_perl_class_lowercase_w() {
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "w",
    };
    parser_i.parse_perl_class();
}

#[test]
fn test_parse_perl_class_uppercase_W() {
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "W",
    };
    parser_i.parse_perl_class();
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'x'")]
fn test_parse_perl_class_invalid_character() {
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "x",
    };
    parser_i.parse_perl_class();
}

