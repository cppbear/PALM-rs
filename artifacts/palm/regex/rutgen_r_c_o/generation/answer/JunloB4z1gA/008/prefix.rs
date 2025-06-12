// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: 0, end: 10 };
    let item = ClassSetItem::Empty(span);
    let _ = item.span();
}

#[test]
fn test_span_literal() {
    let literal = Literal {
        span: Span { start: 10, end: 20 },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_range() {
    let range = ClassSetRange {
        span: Span { start: 20, end: 30 },
        start: Literal::Unicode('a'),
        end: Literal::Unicode('z'),
    };
    let item = ClassSetItem::Range(range);
    let _ = item.span();
}

#[test]
fn test_span_ascii() {
    let ascii_class = ClassAscii {
        span: Span { start: 30, end: 40 },
        kind: ClassAsciiKind::Alnum,
        negated: false,
    };
    let item = ClassSetItem::Ascii(ascii_class);
    let _ = item.span();
}

#[test]
fn test_span_perl() {
    let perl_class = ClassPerl {
        span: Span { start: 40, end: 50 },
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let item = ClassSetItem::Perl(perl_class);
    let _ = item.span();
}

#[test]
fn test_span_unicode() {
    let unicode_class = ClassUnicode {
        span: Span { start: 50, end: 60 },
        negated: false,
        kind: ClassUnicodeKind::Letter,
    };
    let item = ClassSetItem::Unicode(unicode_class);
    let _ = item.span();
}

#[test]
fn test_span_bracketed() {
    let bracketed = ClassBracketed {
        span: Span { start: 60, end: 70 },
        negated: false,
        kind: ClassSet::Normal,
    };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    let _ = item.span();
}

#[test]
fn test_span_union() {
    let union = ClassSetUnion {
        span: Span { start: 70, end: 80 },
        items: vec![],
    };
    let item = ClassSetItem::Union(union);
    let _ = item.span();
}

