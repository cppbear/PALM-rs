// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    let mut depth = 0;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Empty(Span::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal() {
    let mut depth = 1;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Literal(ast::Literal::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_range() {
    let mut depth = 2;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Range(ast::ClassSetRange::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let mut depth = 3;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Perl(ast::ClassPerl::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let mut depth = 4;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Unicode(ast::ClassUnicode::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let mut depth = 5;
    let parser = Parser {
        pos: Cell::new(Position::default()),
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
        pattern: "",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let ast_item = ast::ClassSetItem::Ascii(ast::ClassAscii::default());
    nest_limiter.visit_class_set_item_post(&ast_item).unwrap();
}

