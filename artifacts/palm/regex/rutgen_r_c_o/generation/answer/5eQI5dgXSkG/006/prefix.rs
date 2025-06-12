// Answer 0

#[test]
fn test_perl_class_digit() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\d",
    };
    parser.parse_perl_class();
}

#[test]
fn test_perl_class_negated_digit() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\D",
    };
    parser.parse_perl_class();
}

#[test]
fn test_perl_class_space() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\s",
    };
    parser.parse_perl_class();
}

#[test]
fn test_perl_class_negated_space() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\S",
    };
    parser.parse_perl_class();
}

#[test]
fn test_perl_class_word() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\w",
    };
    parser.parse_perl_class();
}

#[test]
fn test_perl_class_negated_word() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\W",
    };
    parser.parse_perl_class();
}

#[should_panic]
#[test]
fn test_perl_class_invalid_character() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern: r"\x",
    };
    parser.parse_perl_class();
}

