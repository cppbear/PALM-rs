// Answer 0

#[test]
fn test_class_set_span_item_literal() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassSetRange {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassAscii {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassUnicode {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassPerl {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        span: Span,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassSetUnion {
        span: Span,
    }

    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone() };
    let item = ClassSetItem::Literal(literal);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let item = ClassSetItem::Empty(span.clone());
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_range() {
    let span = Span { start: Position(0), end: Position(2) };
    let range = ClassSetRange { span: span.clone() };
    let item = ClassSetItem::Range(range);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_asci() {
    let span = Span { start: Position(0), end: Position(1) };
    let ascii = ClassAscii { span: span.clone() };
    let item = ClassSetItem::Ascii(ascii);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let unicode = ClassUnicode { span: span.clone() };
    let item = ClassSetItem::Unicode(unicode);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let perl = ClassPerl { span: span.clone() };
    let item = ClassSetItem::Perl(perl);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_bracketed() {
    let span = Span { start: Position(0), end: Position(1) };
    let bracketed = ClassBracketed { span: span.clone() };
    let item = ClassSetItem::Bracketed(Box::new(bracketed));
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

#[test]
fn test_class_set_span_item_union() {
    let span = Span { start: Position(0), end: Position(1) };
    let union = ClassSetUnion { span: span.clone() };
    let item = ClassSetItem::Union(union);
    let class_set = ClassSet::Item(item);

    assert_eq!(class_set.span(), &span);
}

