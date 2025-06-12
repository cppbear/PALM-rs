// Answer 0

#[test]
fn test_add_capture_name_unique() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
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
    
    let capture_name = CaptureName {
        span: Span { start: Position(0), end: Position(5) },
        name: "unique_name".to_string(),
        index: 0,
    };
    
    let parser_instance = ParserI::new(&parser, "test_pattern");
    parser_instance.add_capture_name(&capture_name);
}

#[test]
fn test_add_capture_name_duplicate() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
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
    
    let capture_name1 = CaptureName {
        span: Span { start: Position(0), end: Position(5) },
        name: "duplicate_name".to_string(),
        index: 0,
    };
    
    let capture_name2 = CaptureName {
        span: Span { start: Position(6), end: Position(11) },
        name: "duplicate_name".to_string(),
        index: 1,
    };
    
    let parser_instance = ParserI::new(&parser, "test_pattern");
    parser_instance.add_capture_name(&capture_name1);
    parser_instance.add_capture_name(&capture_name2);
}

#[test]
fn test_add_capture_name_boundary_case() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
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

    let mut unique_capture_names = Vec::new();
    for i in 0..999 {
        unique_capture_names.push(CaptureName {
            span: Span { start: Position(i), end: Position(i + 5) },
            name: format!("name_{}", i),
            index: i as u32,
        });
    }
    
    let parser_instance = ParserI::new(&parser, "test_pattern");
    
    for cap in unique_capture_names {
        parser_instance.add_capture_name(&cap);
    }
}

