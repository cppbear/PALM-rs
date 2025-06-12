// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    let span = Span { start: 0, end: 1 };
    let class_set_item = ast::ClassSetItem::Empty(span);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_literal() {
    let literal = ast::Literal::from('a');
    let class_set_item = ast::ClassSetItem::Literal(literal);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_range() {
    let range = ast::ClassSetRange { start: 'a', end: 'z' };
    let class_set_item = ast::ClassSetItem::Range(range);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let ascii_class = ast::ClassAscii::new();
    let class_set_item = ast::ClassSetItem::Ascii(ascii_class);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let unicode_class = ast::ClassUnicode::new();
    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let perl_class = ast::ClassPerl::new();
    let class_set_item = ast::ClassSetItem::Perl(perl_class);
    let parser = Parser {};
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.visit_class_set_item_post(&class_set_item);
}

