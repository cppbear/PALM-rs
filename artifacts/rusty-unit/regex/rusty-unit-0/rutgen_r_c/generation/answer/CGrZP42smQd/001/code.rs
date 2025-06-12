// Answer 0

#[test]
fn test_push_on_empty_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
            end: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
        },
        items: Vec::new(),
    };

    let item = ClassSetItem::Empty(Span {
        start: Position {
            offset: 0,
            line: 1,
            column: 1,
        },
        end: Position {
            offset: 1,
            line: 1,
            column: 2,
        },
    });

    union.push(item.clone());

    assert_eq!(union.items.len(), 1);
    assert_eq!(union.span.start, item.span().start);
    assert_eq!(union.span.end, item.span().end);
}

#[test]
fn test_push_updates_span_on_non_empty_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 8,
                line: 1,
                column: 9,
            },
            end: Position {
                offset: 10,
                line: 1,
                column: 11,
            },
        },
        items: vec![
            ClassSetItem::Empty(Span {
                start: Position {
                    offset: 5,
                    line: 1,
                    column: 6,
                },
                end: Position {
                    offset: 6,
                    line: 1,
                    column: 7,
                },
            }),
        ],
    };

    let item = ClassSetItem::Literal(Literal {
        span: Span {
            start: Position {
                offset: 11,
                line: 1,
                column: 12,
            },
            end: Position {
                offset: 12,
                line: 1,
                column: 13,
            },
        },
    });

    union.push(item.clone());

    assert_eq!(union.items.len(), 2);
    assert_eq!(union.span.start, Position {
        offset: 5,
        line: 1,
        column: 6,
    });
    assert_eq!(union.span.end, Position {
        offset: 12,
        line: 1,
        column: 13,
    });
}

#[test]
fn test_push_with_different_item_types() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
            end: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
        },
        items: Vec::new(),
    };

    let literal_item = ClassSetItem::Literal(Literal {
        span: Span {
            start: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
            end: Position {
                offset: 1,
                line: 1,
                column: 2,
            },
        },
    });

    let range_item = ClassSetItem::Range(ClassSetRange {
        span: Span {
            start: Position {
                offset: 2,
                line: 1,
                column: 3,
            },
            end: Position {
                offset: 4,
                line: 1,
                column: 4,
            },
        },
    });

    union.push(literal_item);
    union.push(range_item);

    assert_eq!(union.items.len(), 2);
    assert_eq!(union.span.start, Position {
        offset: 0,
        line: 1,
        column: 1,
    });
    assert_eq!(union.span.end, Position {
        offset: 4,
        line: 1,
        column: 4,
    });
}

