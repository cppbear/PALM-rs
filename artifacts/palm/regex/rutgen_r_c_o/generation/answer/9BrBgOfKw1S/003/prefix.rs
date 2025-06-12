// Answer 0

#[test]
fn test_visit_class_set_item_pre_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast_item = ast::ClassSetItem::Empty(span);
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_item = ast::ClassSetItem::Literal(Literal('a'));
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_range() {
    let span = Span { start: Position(0), end: Position(2) };
    let ast_item = ast::ClassSetItem::Range(ClassSetRange { start: 'a', end: 'z' });
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_ascii() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_item = ast::ClassSetItem::Ascii(ClassAscii::Alnum);
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_item = ast::ClassSetItem::Unicode(ClassUnicode::Character('ç'));
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_item = ast::ClassSetItem::Perl(ClassPerl::Digit);
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

