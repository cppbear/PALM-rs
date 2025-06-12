// Answer 0

#[test]
fn test_class_set_item_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let item = ClassSetItem::Empty(span.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_literal() {
    let span = Span { start: Position(2), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let item = ClassSetItem::Literal(literal.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_range() {
    let span = Span { start: Position(4), end: Position(5) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'z' };
    let range = ClassSetRange { span: span.clone(), start: start_literal, end: end_literal };
    let item = ClassSetItem::Range(range);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_ascii() {
    let span = Span { start: Position(6), end: Position(7) };
    let ascii = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Word, negated: false };
    let item = ClassSetItem::Ascii(ascii);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_perl() {
    let span = Span { start: Position(8), end: Position(9) };
    let perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let item = ClassSetItem::Perl(perl);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_unicode() {
    let span = Span { start: Position(10), end: Position(11) };
    let unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let item = ClassSetItem::Unicode(unicode);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_bracketed() {
    let span = Span { start: Position(12), end: Position(13) };
    let bracketed = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    assert_eq!(item.span(), &span);
}

#[test]
fn test_class_set_item_union() {
    let span = Span { start: Position(14), end: Position(15) };
    let union = ClassSetUnion { span: span.clone(), items: vec![] };
    let item = ClassSetItem::Union(union);
    assert_eq!(item.span(), &span);
}

