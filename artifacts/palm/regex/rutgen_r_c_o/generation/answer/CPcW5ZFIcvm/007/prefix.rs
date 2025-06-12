// Answer 0

#[test]
fn test_visit_post_empty() {
    let ast = Ast::Empty(Span::new(0, 0));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_literal() {
    let ast = Ast::Literal(ast::Literal::from_string("abc"));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_dot() {
    let ast = Ast::Dot(Span::new(0, 1));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_assertion() {
    let ast = Ast::Assertion(ast::Assertion::new());
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_unicode_class() {
    let ast = Ast::Class(ast::Class::Unicode(ast::UnicodeClass::Letter));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_perl_class() {
    let ast = Ast::Class(ast::Class::Perl(ast::PerlClass::Word));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

#[test]
fn test_visit_post_bracketed_class() {
    let ast = Ast::Class(ast::Class::Bracketed(ast::BracketedClass::new(vec![
        ast::ClassSetItem::Char('a'),
        ast::ClassSetItem::Char('b'),
    ])));
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_post(&ast);
}

