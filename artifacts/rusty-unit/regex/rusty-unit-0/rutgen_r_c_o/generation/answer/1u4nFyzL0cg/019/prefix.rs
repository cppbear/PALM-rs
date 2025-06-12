// Answer 0

#[test]
fn test_parse_set_class_simple_characters() {
    let parser = ParserI {
        parser: Parser { 
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
        },
        pattern: "[abc]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_range() {
    let parser = ParserI {
        parser: Parser { 
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
        },
        pattern: "[a-z]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_nested_classes() {
    let parser = ParserI {
        parser: Parser { 
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
        },
        pattern: "[[[:alpha:]]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_intersection() {
    let parser = ParserI {
        parser: Parser { 
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
        },
        pattern: "[a-c&&[b-z]]",
    };
    parser.parse_set_class();
}

#[test]
#[should_panic]
fn test_parse_set_class_empty() {
    let parser = ParserI {
        parser: Parser { 
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
        },
        pattern: "[",
    };
    parser.parse_set_class();
}

