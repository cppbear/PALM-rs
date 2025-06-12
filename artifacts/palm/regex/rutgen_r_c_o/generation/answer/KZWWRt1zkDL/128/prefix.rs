// Answer 0

#[test]
fn test_parse_escape_assertion_start_text() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let parser = Parser { 
        pos: Cell::new(position), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(true), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\A" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_assertion_end_text() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\z" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_digit() {
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\d" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_space() {
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\s" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_word() {
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\w" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_unicode_class() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\u" };
    
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_unicode_class_long() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
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
        scratch: RefCell::new(String::new())
    };
    let parser_i = ParserI { parser: &parser, pattern: "\\U" };
    
    let _ = parser_i.parse_escape();
}

