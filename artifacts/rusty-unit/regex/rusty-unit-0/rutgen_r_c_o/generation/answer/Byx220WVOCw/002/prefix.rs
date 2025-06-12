// Answer 0

#[test]
fn test_add_duplicate_capture_name_scenario_1() {
    let mut parser = Parser {
        pos: Cell::new(Position { /* initializer */ }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![CaptureName { span: Span { start: 0, end: 1 }, name: "duplicate".into(), index: 0 }]),
        scratch: RefCell::new(String::new()),
    };

    let capture_name = CaptureName { span: Span { start: 2, end: 3 }, name: "duplicate".into(), index: 1 };

    let parser_i = ParserI::new(&parser, "pattern");
    let _ = parser_i.add_capture_name(&capture_name);
}

#[test]
fn test_add_duplicate_capture_name_scenario_2() {
    let mut parser = Parser {
        pos: Cell::new(Position { /* initializer */ }),
        capture_index: Cell::new(1),
        nest_limit: 20,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![
            CaptureName { span: Span { start: 0, end: 1 }, name: "first".into(), index: 0 },
            CaptureName { span: Span { start: 1, end: 2 }, name: "duplicate".into(), index: 1 },
        ]),
        scratch: RefCell::new(String::new()),
    };

    let capture_name = CaptureName { span: Span { start: 3, end: 4 }, name: "duplicate".into(), index: 2 };

    let parser_i = ParserI::new(&parser, "pattern");
    let _ = parser_i.add_capture_name(&capture_name);
}

#[test]
fn test_add_duplicate_capture_name_scenario_3() {
    let mut parser = Parser {
        pos: Cell::new(Position { /* initializer */ }),
        capture_index: Cell::new(2),
        nest_limit: 30,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![
            CaptureName { span: Span { start: 0, end: 1 }, name: "duplicate".into(), index: 0 },
            CaptureName { span: Span { start: 1, end: 1 }, name: "unique".into(), index: 1 },
            CaptureName { span: Span { start: 2, end: 2 }, name: "duplicate".into(), index: 2 },
        ]),
        scratch: RefCell::new(String::new()),
    };

    let capture_name = CaptureName { span: Span { start: 3, end: 4 }, name: "duplicate".into(), index: 3 };

    let parser_i = ParserI::new(&parser, "pattern");
    let _ = parser_i.add_capture_name(&capture_name);
}

