// Answer 0

#[test]
fn test_push_group_with_left_set() {
    let position = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let parser = Parser {
        pos: position,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "(";
    let parser_i = ParserI::new(&parser, pattern);
    let concat = Concat {
        span: Span { start: 0, end: 1 },
        asts: vec![],
    };
    
    let _ = parser_i.push_group(concat);
}

#[test]
fn test_push_group_with_right_group() {
    let position = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let parser = Parser {
        pos: position,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "(a)";
    let parser_i = ParserI::new(&parser, pattern);
    let concat = Concat {
        span: Span { start: 0, end: 3 },
        asts: vec![],
    };

    let _ = parser_i.push_group(concat);
}

#[test]
#[should_panic]
fn test_push_group_panic_on_invalid_character() {
    let position = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let parser = Parser {
        pos: position,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "a(b)";
    let parser_i = ParserI::new(&parser, pattern);
    let concat = Concat {
        span: Span { start: 0, end: 3 },
        asts: vec![],
    };

    let _ = parser_i.push_group(concat);
}

