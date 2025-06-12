// Answer 0

#[test]
fn test_parse_octal_valid_case() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "000";
    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_instance = ParserI {
        parser,
        pattern,
    };
    
    let _result = parser_instance.parse_octal();
}

#[test]
fn test_parse_octal_two_digit_case() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "77";
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    
    let parser_instance = ParserI {
        parser,
        pattern,
    };
    
    let _result = parser_instance.parse_octal();
}

#[test]
fn test_parse_octal_three_digit_case() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "511";
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    
    let parser_instance = ParserI {
        parser,
        pattern,
    };

    let _result = parser_instance.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_exceeding_three_digits() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "5120";
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    
    let parser_instance = ParserI {
        parser,
        pattern,
    };
    
    let _result = parser_instance.parse_octal();
}

