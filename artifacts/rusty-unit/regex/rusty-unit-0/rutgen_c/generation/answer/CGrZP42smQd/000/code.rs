// Answer 0

#[test]
fn test_push_empty_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 0, line: 1, column: 1 },
        },
        items: Vec::new(),
    };
    let item = ClassSetItem::Empty(Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    });

    union.push(item);

    assert_eq!(union.items.len(), 1);
    assert_eq!(union.span.start, union.items[0].span().start);
    assert_eq!(union.span.end, union.items[0].span().end);
}

#[test]
fn test_push_non_empty_union() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 5, line: 1, column: 6 },
        },
        items: vec![ClassSetItem::Empty(Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 5, line: 1, column: 6 },
        })],
    };
    let item = ClassSetItem::Literal(Literal {
        span: Span {
            start: Position { offset: 5, line: 1, column: 6 },
            end: Position { offset: 10, line: 1, column: 11 },
        },
        // Additional fields for Literal would be initialized here
    });

    union.push(item);

    assert_eq!(union.items.len(), 2);
    assert_eq!(union.span.start, union.items[0].span().start);
    assert_eq!(union.span.end, union.items[1].span().end);
}

#[test]
fn test_push_range_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 0, line: 1, column: 1 },
        },
        items: Vec::new(),
    };
    let range = ClassSetRange {
        span: Span {
            start: Position { offset: 1, line: 1, column: 2 },
            end: Position { offset: 4, line: 1, column: 5 },
        },
        // Additional fields for ClassSetRange would be initialized here
    };
    let item = ClassSetItem::Range(range);

    union.push(item);

    assert_eq!(union.items.len(), 1);
    assert_eq!(union.span.start, union.items[0].span().start);
    assert_eq!(union.span.end, union.items[0].span().end);
}

#[test]
fn test_push_multiple_items() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 0, line: 1, column: 1 },
        },
        items: Vec::new(),
    };
    let item1 = ClassSetItem::Ascii(ClassAscii {
        span: Span {
            start: Position { offset: 3, line: 1, column: 4 },
            end: Position { offset: 6, line: 1, column: 7 },
        },
        // Additional fields for ClassAscii would be initialized here
    });
    let item2 = ClassSetItem::Unicode(ClassUnicode {
        span: Span {
            start: Position { offset: 6, line: 1, column: 7 },
            end: Position { offset: 9, line: 1, column: 10 },
        },
        // Additional fields for ClassUnicode would be initialized here
    });

    union.push(item1);
    union.push(item2);

    assert_eq!(union.items.len(), 2);
    assert_eq!(union.span.start, union.items[0].span().start);
    assert_eq!(union.span.end, union.items[1].span().end);
}

