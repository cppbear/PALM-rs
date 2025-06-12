// Answer 0

#[test]
fn test_span_union_with_small_range() {
    let span = Span { start: Position(0), end: Position(1) };
    let union_item = ClassSetItem::Union(ClassSetUnion { span, items: vec![] });
    union_item.span();
}

#[test]
fn test_span_union_with_medium_range() {
    let span = Span { start: Position(0), end: Position(10) };
    let union_item = ClassSetItem::Union(ClassSetUnion { span, items: vec![] });
    union_item.span();
}

#[test]
fn test_span_union_with_zero_range() {
    let span = Span { start: Position(1), end: Position(1) };
    let union_item = ClassSetItem::Union(ClassSetUnion { span, items: vec![] });
    union_item.span();
}

#[test]
fn test_span_union_with_large_range() {
    let span = Span { start: Position(10), end: Position(10) };
    let union_item = ClassSetItem::Union(ClassSetUnion { span, items: vec![] });
    union_item.span();
}

