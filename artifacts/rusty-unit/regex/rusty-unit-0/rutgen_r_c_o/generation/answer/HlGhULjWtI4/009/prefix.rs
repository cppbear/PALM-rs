// Answer 0

#[test]
fn test_class_induct_item_range() {
    let range = ast::ClassSetItem::Range(0..255);
    let class_induct = ClassInduct::Item(&range);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_empty() {
    let empty = ast::ClassSetItem::Empty(Span { start: 0, end: 0 });
    let class_induct = ClassInduct::Item(&empty);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_literal() {
    let literal = ast::ClassSetItem::Literal(Literal::new('a'));
    let class_induct = ClassInduct::Item(&literal);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_ascii() {
    let ascii = ast::ClassSetItem::Ascii(ClassAscii::new("[:alnum:]".to_string()));
    let class_induct = ClassInduct::Item(&ascii);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_unicode() {
    let unicode = ast::ClassSetItem::Unicode(ClassUnicode::new("\\pL".to_string()));
    let class_induct = ClassInduct::Item(&unicode);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_perl() {
    let perl = ast::ClassSetItem::Perl(ClassPerl::new("\\d".to_string()));
    let class_induct = ClassInduct::Item(&perl);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_bracketed() {
    let bracketed = ast::ClassSetItem::Bracketed(Box::new(ClassBracketed::new(vec![])));
    let class_induct = ClassInduct::Item(&bracketed);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

#[test]
fn test_class_induct_item_union() {
    let union = ast::ClassSetItem::Union(ClassSetUnion::new(vec![]));
    let class_induct = ClassInduct::Item(&union);
    let mut formatter = fmt::Formatter::new();
    fmt(&class_induct, &mut formatter);
}

