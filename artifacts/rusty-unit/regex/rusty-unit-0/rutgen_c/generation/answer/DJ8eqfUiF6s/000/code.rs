// Answer 0

#[test]
fn test_span_char_single_line() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "a");
    let span = parser_i.span_char();
    
    assert_eq!(span.start, position);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 2);
}

#[test]
fn test_span_char_newline() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "a\n");
    
    // Mocking the char method to return newline character
    let original_char = parser_i.char;
    parser_i.char = || '\n';
    
    let span = parser_i.span_char();
    
    assert_eq!(span.start, position);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 1);
    
    // Restore original char method 
    parser_i.char = original_char;
}

#[test]
fn test_span_char_empty() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "");
    
    // Mocking the char method to return an empty value
    let original_char = parser_i.char;
    parser_i.char = || '\0';
    
    let span = parser_i.span_char();
    
    assert_eq!(span.start, position);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 1);
    
    // Restore original char method 
    parser_i.char = original_char;
}

