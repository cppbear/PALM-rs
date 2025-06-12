// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let item = ClassSetItem::Empty(span.clone());
    assert_eq!(item.span(), &span);
}

#[test]
fn test_span_literal() {
    let literal = Literal {
        span: Span { start: Position(1), end: Position(2) },
        kind: LiteralKind::Unicode,
        c: 'a',
    };
    let item = ClassSetItem::Literal(literal.clone());
    assert_eq!(item.span(), &literal.span);
}

#[test]
fn test_span_range() {
    let range = ClassSetRange {
        span: Span { start: Position(2), end: Position(5) },
        start: Literal::Unicode('a'),
        end: Literal::Unicode('z'),
    };
    let item = ClassSetItem::Range(range.clone());
    assert_eq!(item.span(), &range.span);
}

#[test]
fn test_span_ascii() {
    let ascii = ClassAscii {
        span: Span { start: Position(3), end: Position(4) },
        kind: ClassAsciiKind::Alnum,
        negated: false,
    };
    let item = ClassSetItem::Ascii(ascii.clone());
    assert_eq!(item.span(), &ascii.span);
}

#[test]
fn test_span_perl() {
    let perl = ClassPerl {
        span: Span { start: Position(4), end: Position(5) },
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let item = ClassSetItem::Perl(perl.clone());
    assert_eq!(item.span(), &perl.span);
}

#[test]
fn test_span_unicode() {
    let unicode = ClassUnicode {
        span: Span { start: Position(5), end: Position(6) },
        negated: false,
        kind: ClassUnicodeKind::Letter,
    };
    let item = ClassSetItem::Unicode(unicode.clone());
    assert_eq!(item.span(), &unicode.span);
}

#[test]
fn test_span_bracketed() {
    let bracketed = ClassBracketed {
        span: Span { start: Position(6), end: Position(7) },
        negated: false,
        kind: ClassSet::Normal,
    };
    let item = ClassSetItem::Bracketed(Box::new(bracketed.clone()));
    assert_eq!(item.span(), &bracketed.span);
}

#[test]
fn test_span_union() {
    let union = ClassSetUnion {
        span: Span { start: Position(7), end: Position(8) },
        items: vec![],
    };
    let item = ClassSetItem::Union(union.clone());
    assert_eq!(item.span(), &union.span);
}

