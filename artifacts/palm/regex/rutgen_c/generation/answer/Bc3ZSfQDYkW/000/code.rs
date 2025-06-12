// Answer 0

#[test]
fn test_union_empty() {
    struct Span;
    let union = ClassSetUnion {
        span: Span,
        items: Vec::new(),
    };
    let result = ClassSet::union(union);
    if let ClassSet::Item(ClassSetItem::Union(ref u)) = result {
        assert_eq!(u.items.len(), 0);
    } else {
        panic!("Expected union result");
    }
}

#[test]
fn test_union_single_literal() {
    struct Span;
    struct Literal;

    let union = ClassSetUnion {
        span: Span,
        items: vec![ClassSetItem::Literal(Literal)],
    };
    let result = ClassSet::union(union);
    if let ClassSet::Item(ClassSetItem::Union(ref u)) = result {
        assert_eq!(u.items.len(), 1);
    } else {
        panic!("Expected union result");
    }
}

#[test]
fn test_union_multiple_items() {
    struct Span;
    struct Literal;
    struct ClassSetRange;

    let union = ClassSetUnion {
        span: Span,
        items: vec![
            ClassSetItem::Literal(Literal),
            ClassSetItem::Range(ClassSetRange),
        ],
    };
    let result = ClassSet::union(union);
    if let ClassSet::Item(ClassSetItem::Union(ref u)) = result {
        assert_eq!(u.items.len(), 2);
    } else {
        panic!("Expected union result");
    }
}

