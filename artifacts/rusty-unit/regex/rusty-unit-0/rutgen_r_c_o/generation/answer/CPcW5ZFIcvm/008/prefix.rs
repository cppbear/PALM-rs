// Answer 0

#[test]
fn test_visit_post_with_assertion() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Assertion(Assertion::default());
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_literal() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Literal(Literal::default());
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_dot() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Dot(Span::default());
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_flags() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Flags(SetFlags::default());
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_empty() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Empty(Span::default());
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_class_unicode() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Class(ast::Class::Unicode(vec![]));
    let _ = nest_limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_with_class_perl() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
    let parser_i = ParserI { parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Class(ast::Class::Perl(vec![]));
    let _ = nest_limiter.visit_post(&ast);
}

