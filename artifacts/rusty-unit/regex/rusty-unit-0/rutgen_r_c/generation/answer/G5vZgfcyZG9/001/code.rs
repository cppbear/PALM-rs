// Answer 0

#[test]
fn test_char_at_valid_index() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new())
    }, pattern);
    
    assert_eq!(parser.char_at(0), 'a');
    assert_eq!(parser.char_at(1), 'b');
    assert_eq!(parser.char_at(2), 'c');
    assert_eq!(parser.char_at(3), 'd');
    assert_eq!(parser.char_at(4), 'e');
}

#[test]
#[should_panic(expected = "expected char at offset 5")]
fn test_char_at_out_of_bounds_high() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new())
    }, pattern);
    
    parser.char_at(5);
}

#[test]
#[should_panic(expected = "expected char at offset 0")]
fn test_char_at_empty_string() {
    let pattern = "";
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new())
    }, pattern);
    
    parser.char_at(0);
}

