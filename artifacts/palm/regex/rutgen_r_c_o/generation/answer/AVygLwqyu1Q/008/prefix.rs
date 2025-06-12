// Answer 0

#[test]
fn test_visit_class_set_item_post_with_empty() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Empty(Span::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_with_literal() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Literal(Literal::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_with_range() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Range(ClassSetRange::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_with_ascii() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Ascii(ClassAscii::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_with_unicode() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Unicode(ClassUnicode::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_with_perl() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser,
        pattern: "test",
    };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    let class_set_item = ast::ClassSetItem::Perl(ClassPerl::default());
    let _ = nest_limiter.visit_class_set_item_post(&class_set_item);
}

