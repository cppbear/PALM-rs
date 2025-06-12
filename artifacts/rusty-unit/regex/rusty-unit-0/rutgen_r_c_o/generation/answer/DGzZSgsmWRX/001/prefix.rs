// Answer 0

#[test]
fn test_check_empty_ast() {
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "",
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Empty(Span::new(0, 0));
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_depth_limit_reached() {
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![GroupState::new()]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "(a(b))",
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Group(Group::new(Box::new(Ast::Literal(Literal::new('a'))), Span::new(0, 0)));
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_simple_literal_ast() {
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "a",
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Literal(Literal::new('a'));
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_high_nesting_depth() {
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![GroupState::new(); 10]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "(a(b(c(d(e(f(g(h(i(j(k(l(m(n(o(p)))))((((((())))))))))))))))))",
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Group(Group::new(Box::new(Ast::Literal(Literal::new('p'))), Span::new(0, 0)));
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_max_nest_limit() {
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 4294967295,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser,
        pattern: "(a(b(c)))",
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Group(Group::new(Box::new(Ast::Group(Group::new(Box::new(Ast::Literal(Literal::new('a'))), Span::new(0, 0)))), Span::new(0, 0)));
    let _ = nest_limiter.check(&ast);
}

