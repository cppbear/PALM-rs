// Answer 0

#[test]
fn test_class_induct_item_empty() {
    let span = Span { start: 0, end: 1 };
    let item = ast::ClassSetItem::Empty(span);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_literal() {
    let span = Span { start: 2, end: 3 };
    let literal = Literal::from('a');
    let item = ast::ClassSetItem::Literal(literal);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_range() {
    let span = Span { start: 4, end: 5 };
    let range = ClassSetRange::new('a', 'z');
    let item = ast::ClassSetItem::Range(range);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_ascii() {
    let span = Span { start: 6, end: 7 };
    let ascii = ClassAscii::Alphanumeric;
    let item = ast::ClassSetItem::Ascii(ascii);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_unicode() {
    let span = Span { start: 8, end: 9 };
    let unicode = ClassUnicode::new("L");
    let item = ast::ClassSetItem::Unicode(unicode);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_perl() {
    let span = Span { start: 10, end: 11 };
    let perl = ClassPerl::Digit;
    let item = ast::ClassSetItem::Perl(perl);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_bracketed() {
    let span = Span { start: 12, end: 13 };
    let bracketed = ClassBracketed::new(vec![ast::ClassSetItem::Literal(Literal::from('b'))]);
    let item = ast::ClassSetItem::Bracketed(Box::new(bracketed));
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

#[test]
fn test_class_induct_item_union() {
    let span = Span { start: 14, end: 15 };
    let union = ClassSetUnion::new(vec![ast::ClassSetItem::Literal(Literal::from('c'))]);
    let item = ast::ClassSetItem::Union(union);
    let induct = ClassInduct::Item(&item);
    let mut formatter = std::fmt::Formatter::new();
    induct.fmt(&mut formatter);
}

