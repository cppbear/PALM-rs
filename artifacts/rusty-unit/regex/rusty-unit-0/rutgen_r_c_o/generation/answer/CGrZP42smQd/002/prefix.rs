// Answer 0

#[test]
fn test_push_non_empty_union_with_literal() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 1, line: 1, column: 2 },
        },
        items: vec![
            ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } } }),
        ],
    };
    let item = ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 1, line: 1, column: 2 }, end: Position { offset: 2, line: 1, column: 3 } } });
    union.push(item);
}

#[test]
fn test_push_non_empty_union_with_range() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 5, line: 1, column: 6 },
        },
        items: vec![
            ClassSetItem::Range(ClassSetRange { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 5, line: 1, column: 6 } } }),
        ],
    };
    let item = ClassSetItem::Range(ClassSetRange { span: Span { start: Position { offset: 5, line: 1, column: 6 }, end: Position { offset: 10, line: 1, column: 11 } } });
    union.push(item);
}

#[test]
fn test_push_non_empty_union_with_bracketed() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 3, line: 1, column: 4 },
        },
        items: vec![
            ClassSetItem::Bracketed(Box::new(ClassBracketed { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 3, line: 1, column: 4 } } })),
        ],
    };
    let item = ClassSetItem::Bracketed(Box::new(ClassBracketed { span: Span { start: Position { offset: 3, line: 1, column: 4 }, end: Position { offset: 6, line: 1, column: 7 } } }));
    union.push(item);
}

#[test]
fn test_push_non_empty_union_with_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 6, line: 1, column: 7 },
        },
        items: vec![
            ClassSetItem::Union(ClassSetUnion { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 6, line: 1, column: 7 } }, items: vec![] }),
        ],
    };
    let item = ClassSetItem::Union(ClassSetUnion { span: Span { start: Position { offset: 6, line: 1, column: 7 }, end: Position { offset: 12, line: 1, column: 8 } }, items: vec![] });
    union.push(item);
}

#[test]
fn test_push_large_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 1000, line: 500, column: 100 },
        },
        items: vec![
            ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } } }),
            ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 1, line: 1, column: 2 }, end: Position { offset: 2, line: 1, column: 3 } } }),
            ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 2, line: 1, column: 3 }, end: Position { offset: 3, line: 1, column: 4 } } }),
        ],
    };
    let item = ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 1000, line: 500, column: 100 }, end: Position { offset: 1001, line: 500, column: 101 } } });
    union.push(item);
}

#[should_panic]
#[test]
fn test_push_with_empty_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 0, line: 1, column: 1 },
        },
        items: vec![],
    };
    let item = ClassSetItem::Literal(Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } } });
    union.push(item);
}

