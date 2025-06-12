// Answer 0

#[test]
fn test_class_set_item_empty_span() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_set_item = ClassSetItem::Empty(span.clone());
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_literal_span() {
    let span = Span { start: Position(1), end: Position(2) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let class_set_item = ClassSetItem::Literal(literal);
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_range_span() {
    let span = Span { start: Position(2), end: Position(3) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('z'), c: 'z' };
    let class_set_range = ClassSetRange { span: span.clone(), start: start_literal, end: end_literal };
    let class_set_item = ClassSetItem::Range(class_set_range);
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_ascii_span() {
    let span = Span { start: Position(3), end: Position(4) };
    let ascii = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Alnum, negated: false };
    let class_set_item = ClassSetItem::Ascii(ascii);
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_unicode_span() {
    let span = Span { start: Position(4), end: Position(5) };
    let unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let class_set_item = ClassSetItem::Unicode(unicode);
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_perl_span() {
    let span = Span { start: Position(5), end: Position(6) };
    let perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let class_set_item = ClassSetItem::Perl(perl);
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_bracketed_span() {
    let span = Span { start: Position(6), end: Position(7) };
    let class_bracketed = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let class_set_item = ClassSetItem::Bracketed(Box::new(class_bracketed));
    assert_eq!(class_set_item.span(), &span);
}

#[test]
fn test_class_set_item_union_span() {
    let span = Span { start: Position(7), end: Position(8) };
    let union = ClassSetUnion { span: span.clone(), items: vec![] };
    let class_set_item = ClassSetItem::Union(union);
    assert_eq!(class_set_item.span(), &span);
}

