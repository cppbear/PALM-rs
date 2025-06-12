// Answer 0

#[test]
fn test_visit_class_set_item_pre_empty() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Empty(Span::new(0, 0));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_literal() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Literal(ast::Literal::from_char('a'));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_range() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let range = ast::ClassSetRange { start: ast::Literal::from_char('a'), end: ast::Literal::from_char('z') };
    let ast = ast::ClassSetItem::Range(range);
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_ascii() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii::Alnum);
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_unicode() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Unicode(ast::ClassUnicode::Property("L".to_string()));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_perl() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl::Digit);
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_union() {
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: String::new() };
    let ast = ast::ClassSetItem::Union(ast::ClassSetUnion::new(vec![]));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

