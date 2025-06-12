// Answer 0

#[test]
fn test_parse_set_class_range_valid_case1() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start_pos, end_pos);
    
    let lit1 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'a' };
    let lit2 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'b' };
    
    let prim1 = Primitive::Literal(lit1);
    let prim2 = Primitive::Literal(lit2);
    
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 50,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "a-b" };
    
    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_valid_case2() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 1, line: 1, column: 2 };
    let span_prim1 = Span::new(start_pos, end_pos);
    
    let lit1 = Literal { span: span_prim1.clone(), kind: ast::LiteralKind::Verbatim, c: 'c' };
    let lit2 = Literal { span: span_prim1.clone(), kind: ast::LiteralKind::Verbatim, c: 'd' };
    
    let prim1 = Primitive::Literal(lit1);
    let prim2 = Primitive::Literal(lit2);
    
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(1),
        nest_limit: 75,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![ast::Comment::new()]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "c-d" };
    
    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid_case1() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start_pos, end_pos);
    
    let lit1 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'x' };
    let lit2 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'y' };
    
    let prim1 = Primitive::Literal(lit1);
    let prim2 = Primitive::Literal(lit2); // This should lead to an invalid range due to ordering.
    
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 100,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "x-y" };
    
    parser_instance.parse_set_class_range();
}

#[test]
#[should_panic]
fn test_parse_set_class_range_eof_case() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    
    let lit1 = Literal { span: Span::new(start_pos, start_pos), kind: ast::LiteralKind::Verbatim, c: 'p' };

    let prim1 = Primitive::Literal(lit1);
    
    let mut parser = Parser {
        pos: Cell::new(start_pos),
        capture_index: Cell::new(0),
        nest_limit: 50,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "p-" };
    
    // Triggering eof condition directly
    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_multiple_valid_cases() {
    let test_cases = vec![
        ('e', 'f'),
        ('g', 'h'),
        ('i', 'j'),
    ];

    for (start_char, end_char) in test_cases {
        let start_pos = Position { offset: 0, line: 1, column: 1 };
        let end_pos = Position { offset: 1, line: 1, column: 2 };
        let span = Span::new(start_pos, end_pos);
        
        let lit1 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: start_char };
        let lit2 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: end_char };

        let prim1 = Primitive::Literal(lit1);
        let prim2 = Primitive::Literal(lit2);
        
        let mut parser = Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 50,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        };

        let parser_instance = ParserI { parser: &parser, pattern: &format!("{}-{}", start_char, end_char) };

        parser_instance.parse_set_class_range();
    }
}

