// Answer 0

#[test]
fn test_union_empty() {
    let ast = ClassSetUnion {
        span: Span::new(0, 0),
        items: Vec::new(),
    };
    union(ast);
}

#[test]
fn test_union_single_literal() {
    let ast = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![ClassSetItem::Literal(Literal::new('a'))],
    };
    union(ast);
}

#[test]
fn test_union_multiple_literals() {
    let ast = ClassSetUnion {
        span: Span::new(0, 5),
        items: vec![
            ClassSetItem::Literal(Literal::new('b')),
            ClassSetItem::Literal(Literal::new('c')),
            ClassSetItem::Literal(Literal::new('d')),
        ],
    };
    union(ast);
}

#[test]
fn test_union_single_range() {
    let ast = ClassSetUnion {
        span: Span::new(0, 3),
        items: vec![ClassSetItem::Range(ClassSetRange::new(1, 5))],
    };
    union(ast);
}

#[test]
fn test_union_multiple_ranges() {
    let ast = ClassSetUnion {
        span: Span::new(0, 10),
        items: vec![
            ClassSetItem::Range(ClassSetRange::new(1, 5)),
            ClassSetItem::Range(ClassSetRange::new(6, 10)),
        ],
    };
    union(ast);
}

#[test]
fn test_union_with_ascii() {
    let ast = ClassSetUnion {
        span: Span::new(0, 4),
        items: vec![ClassSetItem::Ascii(ClassAscii::new("alnum"))],
    };
    union(ast);
}

#[test]
fn test_union_mixed_items() {
    let ast = ClassSetUnion {
        span: Span::new(0, 8),
        items: vec![
            ClassSetItem::Literal(Literal::new('e')),
            ClassSetItem::Range(ClassSetRange::new(5, 10)),
            ClassSetItem::Ascii(ClassAscii::new("punct")),
        ],
    };
    union(ast);
}

#[test]
fn test_union_large_item_count() {
    let ast = ClassSetUnion {
        span: Span::new(0, 20),
        items: (0..10).map(|i| ClassSetItem::Literal(Literal::new((b'a' + i as u8) as char))).collect(),
    };
    union(ast);
}

