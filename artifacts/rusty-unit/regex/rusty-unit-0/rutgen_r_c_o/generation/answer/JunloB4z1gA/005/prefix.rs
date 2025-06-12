// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(1), end: Position(2) };
    let item = ClassSetItem::Empty(span);
    item.span();
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(1), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let item = ClassSetItem::Literal(literal);
    item.span();
}

#[test]
fn test_span_range() {
    let span = Span { start: Position(2), end: Position(4) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Byte(0x61), c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Byte(0x7a), c: 'z' };
    let range = ClassSetRange { span: span.clone(), start: start_literal, end: end_literal };
    let item = ClassSetItem::Range(range);
    item.span();
}

#[test]
fn test_span_ascii() {
    let span = Span { start: Position(1), end: Position(5) };
    let ascii_class = ClassAscii { span: span.clone(), kind: ClassAsciiKind::Alnum, negated: false };
    let item = ClassSetItem::Ascii(ascii_class);
    item.span();
}

#[test]
fn test_span_perl() {
    let span = Span { start: Position(1), end: Position(6) };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let item = ClassSetItem::Perl(perl_class);
    item.span();
}

#[test]
fn test_span_unicode() {
    let span = Span { start: Position(1), end: Position(7) };
    let unicode_class = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let item = ClassSetItem::Unicode(unicode_class);
    item.span();
}

#[test]
fn test_span_bracketed() {
    let span = Span { start: Position(1), end: Position(8) };
    let bracketed = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    item.span();
}

#[test]
fn test_span_union() {
    let span = Span { start: Position(1), end: Position(9) };
    let union = ClassSetUnion { span: span.clone(), items: Vec::new() };
    let item = ClassSetItem::Union(union);
    item.span();
}

