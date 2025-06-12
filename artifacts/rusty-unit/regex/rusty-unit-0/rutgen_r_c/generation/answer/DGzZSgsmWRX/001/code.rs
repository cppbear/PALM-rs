// Answer 0

#[test]
fn test_nest_limiter_check_with_empty_ast() {
    let pattern = ".*"; // example pattern
    let spine = Span::default(); // providing a default span for testing
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10, // arbitrary limit for testing
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter::new(&parser);
    let ast = Ast::Empty(spine); // creating an empty AST

    let result = nest_limiter.check(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_nest_limiter_check_with_literal_ast() {
    let pattern = "abc"; // example pattern
    let spine = Span::default(); // default span
    let literal = Literal::new("a"); // creating a literal 

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10, // arbitrary limit for testing
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter::new(&parser);
    let ast = Ast::Literal(literal); // creating a literal AST

    let result = nest_limiter.check(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_nest_limiter_check_with_depth_limit() {
    let pattern = "(a(b(c)))"; // complex pattern to test depth
    let spine = Span::default(); // default span

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 2, // limit set to 2 to trigger panic
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter::new(&parser);
    let ast = Ast::Group(Group::new(vec![
        Ast::Literal(Literal::new("a")),
        Ast::Group(Group::new(vec![
            Ast::Literal(Literal::new("b")),
            Ast::Group(Group::new(vec![Ast::Literal(Literal::new("c"))])),
        ])),
    ]));

    let result = nest_limiter.check(&ast);
    assert!(result.is_err()); // expecting an error due to exceeding depth limit
}

#[test]
fn test_nest_limiter_check_with_dot_ast() {
    let pattern = ".*"; // example pattern
    let spine = Span::default(); // default span

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10, // arbitrary limit for testing
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter::new(&parser);
    let ast = Ast::Dot(spine); // creating an AST with the dot

    let result = nest_limiter.check(&ast);
    assert!(result.is_ok());
}

