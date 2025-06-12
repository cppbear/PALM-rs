// Answer 0

#[test]
fn test_parse_hex_with_x_char_and_no_space() {
    let pattern = "\\x";
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0), 
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
        },
        pattern,
    };

    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_x_char_and_no_space_but_with_invalid_digit() {
    let pattern = "\\xG";
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0), 
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
        },
        pattern,
    };

    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_u_char_and_no_space() {
    let pattern = "\\u";
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0), 
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
        },
        pattern,
    };

    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_U_char_and_no_space() {
    let pattern = "\\U";
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0), 
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
        },
        pattern,
    };

    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_x_char_followed_by_invalid_sequence() {
    let pattern = "\\x{}";
    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0), 
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
        },
        pattern,
    };

    parser.parse_hex();
}

