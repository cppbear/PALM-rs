// Answer 0

#[test]
fn test_into_item_empty_union() {
    let span = Span { 
        start: Position { byte: 0 }, 
        end: Position { byte: 0 } 
    };
    let union = ClassSetUnion { 
        span, 
        items: vec![] 
    };
    let result = union.clone().into_item();
    assert_eq!(result, ClassSetItem::Empty(span));
}

#[test]
fn test_into_item_single_item() {
    let span = Span { 
        start: Position { byte: 0 }, 
        end: Position { byte: 1 } 
    };
    let literal = Literal('a');
    let union = ClassSetUnion { 
        span, 
        items: vec![ClassSetItem::Literal(literal)] 
    };
    let result = union.clone().into_item();
    assert_eq!(result, ClassSetItem::Literal(literal));
}

#[test]
fn test_into_item_multiple_items() {
    let span = Span { 
        start: Position { byte: 0 }, 
        end: Position { byte: 2 } 
    };
    let union = ClassSetUnion { 
        span, 
        items: vec![
            ClassSetItem::Literal(Literal('a')),
            ClassSetItem::Literal(Literal('b'))
        ] 
    };
    let result = union.clone().into_item();
    if let ClassSetItem::Union(ref extracted_union) = result {
        assert_eq!(extracted_union.items.len(), 2);
        assert_eq!(extracted_union.span, span);
    } else {
        panic!("Expected a Union variant");
    }
}

