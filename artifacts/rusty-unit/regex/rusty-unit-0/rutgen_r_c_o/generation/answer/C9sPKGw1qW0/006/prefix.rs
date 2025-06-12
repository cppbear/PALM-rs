// Answer 0

#[test]
fn test_visit_class_set_item_post_with_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = ast::ClassSetItem::Empty(Span::default());
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_literal() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let literal = Literal { span: Span::default(), kind: LiteralKind::Verbatim, c: 'a' };
    let ast = ast::ClassSetItem::Literal(literal);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_range() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let start_literal = Literal { span: Span::default(), kind: LiteralKind::Verbatim, c: 'a' };
    let end_literal = Literal { span: Span::default(), kind: LiteralKind::Verbatim, c: 'z' };
    let range = ClassSetRange { span: Span::default(), start: start_literal, end: end_literal };
    let ast = ast::ClassSetItem::Range(range);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
#[should_panic]
fn test_visit_class_set_item_post_with_range_start_panic() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let start_literal = Literal { span: Span::default(), kind: LiteralKind::Verbatim, c: 'ÿ' }; // valid but non-integer
    let end_literal = Literal { span: Span::default(), kind: LiteralKind::Verbatim, c: 'z' };
    let range = ClassSetRange { span: Span::default(), start: start_literal, end: end_literal };
    let ast = ast::ClassSetItem::Range(range);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_ascii() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_ascii = ClassAscii { span: Span::default(), kind: ClassAsciiKind::Alnum, negated: false };
    let ast = ast::ClassSetItem::Ascii(class_ascii);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_unicode() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_unicode = ClassUnicode { span: Span::default(), negated: false, kind: ClassUnicodeKind::OneLetter('λ') };
    let ast = ast::ClassSetItem::Unicode(class_unicode);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_perl() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_perl = ClassPerl { span: Span::default(), kind: ClassPerlKind::Digit, negated: false };
    let ast = ast::ClassSetItem::Perl(class_perl);
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_bracketed() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_bracketed = ClassBracketed { span: Span::default(), negated: false, kind: ClassSet::NormalUnion(vec![]) };
    let ast = ast::ClassSetItem::Bracketed(Box::new(class_bracketed));
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_with_union() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_union = ClassSetUnion { span: Span::default(), items: vec![] };
    let ast = ast::ClassSetItem::Union(class_union);
    writer.visit_class_set_item_post(&ast).unwrap();
}

