// Answer 0

#[test]
fn test_parse_escape_octal_0() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\0" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_1() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\1" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_2() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\2" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_3() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\3" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_4() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\4" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_5() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\5" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_6() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\6" };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_octal_7() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\7" };
    let _ = parser_i.parse_escape();
}

