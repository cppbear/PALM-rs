// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_char() {
    // Setup
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Literal(span.clone());
    let concat = Ast::Concat { span, asts: vec![ast.clone()] };
    let parser_state = Parser {
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
    let parser_instance = ParserI { parser: &parser_state, pattern: "{}" };

    // Call the function with invalid char
    let _ = parser_instance.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_pop_empty() {
    // Setup
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = Ast::Concat { span, asts: vec![] }; // Empty asts
    let parser_state = Parser {
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
    let parser_instance = ParserI { parser: &parser_state, pattern: "{}" };

    // Call the function expecting it to return an error
    let _ = parser_instance.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_bump_space_false() {
    // Setup
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Literal(span.clone());
    let concat = Ast::Concat { span, asts: vec![ast.clone()] };
    let parser_state = Parser {
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
    let parser_instance = ParserI { parser: &parser_state, pattern: "{}" };

    // Call the function with bump_and_bump_space() set to false
    let _ = parser_instance.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_repetition_count_closed() {
    // Setup
    let position = Position { offset: 0, line: 1, column: 1 };
    let ast = Ast::Literal(Span::new(position, position));
    let concat = Ast::Concat { span: ast.span().clone(), asts: vec![ast] };
    let parser_state = Parser {
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
        scratch: RefCell::new(String::from("1,}")), // simulate valid inputs
    };
    let parser_instance = ParserI { parser: &parser_state, pattern: "{1,}" };

    // Call function expecting it to process correctly but the end will cause an issue
    let _ = parser_instance.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty() {
    // Setup
    let position = Position { offset: 0, line: 1, column: 1 };
    let ast = Ast::Literal(Span::new(position, position));
    let concat = Ast::Concat { span: ast.span().clone(), asts: vec![ast] };
    let parser_state = Parser {
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
    let parser_instance = ParserI { parser: &parser_state, pattern: "{1,}" };

    // Call the function expecting it to handle empty input without repetition
    let _ = parser_instance.parse_counted_repetition(concat);
}

