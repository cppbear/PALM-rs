// Answer 0

#[test]
fn test_parse_escape_perl_class_d() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\d";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_D() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\D";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_s() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\s";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_S() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\S";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_w() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\w";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_W() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let input = r"\W";
    let parser_instance = ParserI { parser: &parser, pattern: input };
    let _ = parser_instance.parse_escape();
}

