// Answer 0

#[test]
fn test_next_capture_index_increment() {
    let initial_index: u32 = 0;
    let span = Span { start: 0, end: 1 };
    
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(initial_index),
        nest_limit: 1000,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(parser, "test_pattern");
    let result = parser_i.next_capture_index(span).unwrap();
    assert_eq!(result, 1);
}

#[test]
#[should_panic(expected = "CaptureLimitExceeded")]
fn test_next_capture_index_exceed_limit() {
    let initial_index: u32 = u32::MAX; // Set to maximum value to trigger panic
    let span = Span { start: 0, end: 1 };
    
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(initial_index),
        nest_limit: 1000,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(parser, "test_pattern");
    parser_i.next_capture_index(span).unwrap();
}

#[test]
fn test_next_capture_index_successive_calls() {
    let initial_index: u32 = 1;
    let span = Span { start: 0, end: 1 };
    
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(initial_index),
        nest_limit: 1000,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(parser, "test_pattern");

    let first_call = parser_i.next_capture_index(span).unwrap();
    assert_eq!(first_call, 2);
    
    let second_call = parser_i.next_capture_index(span).unwrap();
    assert_eq!(second_call, 3);
}

