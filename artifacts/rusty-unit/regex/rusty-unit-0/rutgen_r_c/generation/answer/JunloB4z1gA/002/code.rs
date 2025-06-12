// Answer 0

#[test]
fn test_class_set_item_span_empty() {
    let span = Span { start: 0, end: 1 };
    let item = ClassSetItem::Empty(span.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_literal() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let item = ClassSetItem::Literal(literal);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_range() {
    let span = Span { start: 0, end: 2 };
    let range = ClassSetRange { span: span.clone(), start: Literal::Byte(97), end: Literal::Byte(122) };
    let item = ClassSetItem::Range(range);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_ascii() {
    let span = Span { start: 0, end: 1 };
    let ascii = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Alnum, negated: false };
    let item = ClassSetItem::Ascii(ascii);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_perl() {
    let span = Span { start: 0, end: 1 };
    let perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let item = ClassSetItem::Perl(perl);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_unicode() {
    let span = Span { start: 0, end: 1 };
    let unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let item = ClassSetItem::Unicode(unicode);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_bracketed() {
    let span = Span { start: 0, end: 5 };
    let bracketed = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_span_union() {
    let span = Span { start: 0, end: 3 };
    let union = ClassSetUnion { span: span.clone(), items: vec![] };
    let item = ClassSetItem::Union(union);
    assert_eq!(item.span(), &span);
}

