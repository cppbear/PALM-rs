// Answer 0

#[test]
fn test_visit_class_set_item_post_bracketed_depth_2() {
    let bracketed_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {}));
    let parser = ParserI { parser: Parser { pos: Cell::new(Position::new()), capture_index: Cell::new(0), nest_limit: 2, octal: true, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.depth = 2;
    nest_limiter.visit_class_set_item_post(&bracketed_item);
}

#[test]
fn test_visit_class_set_item_post_bracketed_depth_1() {
    let bracketed_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {}));
    let parser = ParserI { parser: Parser { pos: Cell::new(Position::new()), capture_index: Cell::new(0), nest_limit: 2, octal: true, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.depth = 1;
    nest_limiter.visit_class_set_item_post(&bracketed_item);
}

#[test]
fn test_visit_class_set_item_post_bracketed_depth_0() {
    let bracketed_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {}));
    let parser = ParserI { parser: Parser { pos: Cell::new(Position::new()), capture_index: Cell::new(0), nest_limit: 2, octal: true, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.depth = 0;
    nest_limiter.visit_class_set_item_post(&bracketed_item);
}

