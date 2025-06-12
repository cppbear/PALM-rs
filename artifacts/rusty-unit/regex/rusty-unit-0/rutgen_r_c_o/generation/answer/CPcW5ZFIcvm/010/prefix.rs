// Answer 0

#[test]
fn test_visit_post_literal() {
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
    let parser_i = ParserI {
        parser,
        pattern: "abc",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Literal(Literal::new("a", Span::default()));
    nest_limiter.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_dot() {
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
    let parser_i = ParserI {
        parser,
        pattern: ".*",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Dot(Span::default());
    nest_limiter.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion() {
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
    let parser_i = ParserI {
        parser,
        pattern: "^abc$",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Assertion(Assertion::new("^", Span::default()));
    nest_limiter.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_unicode_class() {
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
    let parser_i = ParserI {
        parser,
        pattern: "\\u{1F600}",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Class(Class::Unicode(vec![]));
    nest_limiter.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_perl_class() {
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
    let parser_i = ParserI {
        parser,
        pattern: "\\p{L}",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast = Ast::Class(Class::Perl(vec![]));
    nest_limiter.visit_post(&ast).unwrap();
}

