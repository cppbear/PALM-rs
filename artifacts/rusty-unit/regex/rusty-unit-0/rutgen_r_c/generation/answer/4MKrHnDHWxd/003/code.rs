// Answer 0

#[test]
fn test_into_item_empty_union() {
    let span = Span { start: Position::new(0), end: Position::new(0) };
    let empty_union = ClassSetUnion { span, items: Vec::new() };
    let result = empty_union.into_item();
    assert_eq!(result, ClassSetItem::Empty(span));
}

#[test]
fn test_into_item_single_item() {
    let span = Span { start: Position::new(0), end: Position::new(3) };
    let literal = ClassSetItem::Literal(Literal::new('a'));
    let single_item_union = ClassSetUnion { span, items: vec![literal.clone()] };
    let result = single_item_union.into_item();
    assert_eq!(result, literal);
}

#[test]
fn test_into_item_multiple_items() {
    let span = Span { start: Position::new(0), end: Position::new(5) };
    let item1 = ClassSetItem::Literal(Literal::new('a'));
    let item2 = ClassSetItem::Literal(Literal::new('b'));
    let mut multi_item_union = ClassSetUnion { span, items: vec![item1.clone(), item2.clone()] };
    let result = multi_item_union.into_item();
    if let ClassSetItem::Union(result_union) = result {
        assert_eq!(result_union.items.len(), 2);
        assert_eq!(result_union.items[0], item1);
        assert_eq!(result_union.items[1], item2);
    } else {
        panic!("Expected a ClassSetItem::Union, but got a different variant.");
    }
}

