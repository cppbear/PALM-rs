// Answer 0

#[test]
fn test_parse_octal_valid_single_digit() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(span_start, span_end);

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

    let parser_i = ParserI {
        parser,
        pattern: "0abc",
    };

    parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_double_digit() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(span_start, span_end);

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

    let parser_i = ParserI {
        parser,
        pattern: "77xyz",
    };

    parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_triple_digit() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(span_start, span_end);

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

    let parser_i = ParserI {
        parser,
        pattern: "376efg",
    };

    parser_i.parse_octal();
} 

#[test]
#[should_panic]
fn test_parse_octal_invalid_character_high() {
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
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "8abc",
    };

    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_character_low() {
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
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "aabc",
    };

    parser_i.parse_octal();
}

