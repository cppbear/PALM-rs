// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(5) };
    let item = ClassSetItem::Empty(span.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(1), end: Position(2) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let item = ClassSetItem::Literal(literal);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_range() {
    let span = Span { start: Position(0), end: Position(3) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('z'), c: 'z' };
    let range = ClassSetRange { span: span.clone(), start: start_literal, end: end_literal };
    let item = ClassSetItem::Range(range);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_ascii() {
    let span = Span { start: Position(2), end: Position(4) };
    let ascii = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Alnum, negated: false };
    let item = ClassSetItem::Ascii(ascii);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_perl() {
    let span = Span { start: Position(3), end: Position(7) };
    let perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let item = ClassSetItem::Perl(perl);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: Position(0), end: Position(6) };
    let unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let item = ClassSetItem::Unicode(unicode);
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_bracketed() {
    let span = Span { start: Position(1), end: Position(9) };
    let bracketed = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_union() {
    let span = Span { start: Position(0), end: Position(10) };
    let union = ClassSetUnion { span: span.clone(), items: vec![] };
    let item = ClassSetItem::Union(union);
    assert_eq!(item.span(), &span);
}

