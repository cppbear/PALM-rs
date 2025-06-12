// Answer 0

#[test]
fn test_visit_post_repetition_depth_1() {
    let ast = ast::Ast::Repetition(ast::Repetition {});
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 2,
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
    nest_limiter.depth = 1;
    nest_limiter.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition_depth_2() {
    let ast = ast::Ast::Repetition(ast::Repetition {});
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 2,
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
    nest_limiter.depth = 2;
    nest_limiter.visit_post(&ast).unwrap();
}

