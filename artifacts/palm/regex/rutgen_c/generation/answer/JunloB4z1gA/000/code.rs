// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let item = ClassSetItem::Empty(span.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let item = ClassSetItem::Literal(literal);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_range() {
    let span = Span { start: Position(0), end: Position(2) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' };
    let range = ClassSetRange { span: span.clone(), start: start_literal, end: end_literal };
    let item = ClassSetItem::Range(range);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_ascii() {
    let span = Span { start: Position(3), end: Position(4) };
    let ascii_class = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Alnum, negated: false };
    let item = ClassSetItem::Ascii(ascii_class);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_perl() {
    let span = Span { start: Position(5), end: Position(6) };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let item = ClassSetItem::Perl(perl_class);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: Position(7), end: Position(8) };
    let unicode_class = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let item = ClassSetItem::Unicode(unicode_class);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_bracketed() {
    let span = Span { start: Position(9), end: Position(10) };
    let bracketed_class = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let item = ClassSetItem::Bracketed(Box::new(bracketed_class));
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_union() {
    let span = Span { start: Position(11), end: Position(12) };
    let items = vec![ClassSetItem::Empty(span.clone())];
    let union_class = ClassSetUnion { span: span.clone(), items };
    let item = ClassSetItem::Union(union_class);
    assert_eq!(item.span(), &span);
}

