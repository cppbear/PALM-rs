// Answer 0

#[test]
fn test_span_bracketed_class_set_negated_false() {
    let span = Span { start: Position(0), end: Position(10) };
    let class_set = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Normal,
    };
    let item = ClassSetItem::Bracketed(Box::new(class_set));
    let _ = item.span();
}

#[test]
fn test_span_bracketed_class_set_negated_true() {
    let span = Span { start: Position(0), end: Position(10) };
    let class_set = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::Normal,
    };
    let item = ClassSetItem::Bracketed(Box::new(class_set));
    let _ = item.span();
}

#[test]
fn test_span_empty_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let item = ClassSetItem::Empty(span);
    let _ = item.span();
}

#[test]
fn test_span_literal_class_set() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::Unicode,
        c: 'a',
    };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_range_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let start_literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::Unicode,
        c: 'a',
    };
    let end_literal = Literal {
        span: Span { start: Position(1), end: Position(2) },
        kind: LiteralKind::Unicode,
        c: 'z',
    };
    let range = ClassSetRange {
        span,
        start: start_literal,
        end: end_literal,
    };
    let item = ClassSetItem::Range(range);
    let _ = item.span();
}

#[test]
fn test_span_ascii_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let ascii_class = ClassAscii {
        span,
        kind: ClassAsciiKind::Alnum,
        negated: false,
    };
    let item = ClassSetItem::Ascii(ascii_class);
    let _ = item.span();
}

#[test]
fn test_span_unicode_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let unicode_class = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::Uppercase,
    };
    let item = ClassSetItem::Unicode(unicode_class);
    let _ = item.span();
}

#[test]
fn test_span_perl_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let perl_class = ClassPerl {
        span,
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let item = ClassSetItem::Perl(perl_class);
    let _ = item.span();
}

#[test]
fn test_span_union_class_set() {
    let span = Span { start: Position(0), end: Position(10) };
    let union = ClassSetUnion {
        span,
        items: vec![],
    };
    let item = ClassSetItem::Union(union);
    let _ = item.span();
}

