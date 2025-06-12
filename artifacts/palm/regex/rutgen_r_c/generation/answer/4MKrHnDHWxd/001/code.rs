// Answer 0

#[test]
fn test_into_item_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let union = ClassSetUnion { span, items: Vec::new() };
    if let ClassSetItem::Empty(_) = union.into_item() {
        // Test passed
    } else {
        panic!("Expected ClassSetItem::Empty for empty union");
    }
}

#[test]
fn test_into_item_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = ClassSetItem::Literal(Literal::Character('a'));
    let union = ClassSetUnion { span, items: vec![literal.clone()] };
    if let ClassSetItem::Literal(item) = union.into_item() {
        assert_eq!(item, literal);
    } else {
        panic!("Expected ClassSetItem::Literal for single item union");
    }
}

#[test]
fn test_into_item_multiple() {
    let span = Span { start: Position(0), end: Position(1) };
    let item1 = ClassSetItem::Literal(Literal::Character('a'));
    let item2 = ClassSetItem::Literal(Literal::Character('b'));
    let union = ClassSetUnion { span, items: vec![item1.clone(), item2.clone()] };
    if let ClassSetItem::Union(_) = union.into_item() {
        // Test passed
    } else {
        panic!("Expected ClassSetItem::Union for multiple items union");
    }
}

