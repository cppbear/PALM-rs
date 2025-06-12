// Answer 0

#[test]
fn test_push_non_empty_union() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal {
        pub span: Span,
        // Additional fields can go here
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassSetRange {
        pub span: Span,
        // Additional fields can go here
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassAscii {
        pub span: Span,
        // Additional fields can go here
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassUnicode {
        pub span: Span,
        // Additional fields can go here
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassPerl {
        pub span: Span,
        // Additional fields can go here
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        pub span: Span,
        // Additional fields can go here
    }

    let initial_span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    
    let mut union = ClassSetUnion {
        span: initial_span,
        items: vec![],
    };

    let new_span = Span {
        start: Position { offset: 5, line: 1, column: 6 },
        end: Position { offset: 10, line: 1, column: 11 },
    };

    let item = ClassSetItem::Literal(Literal { span: new_span });
    
    union.push(item.clone());

    assert_eq!(union.items.len(), 1);
    assert_eq!(union.items[0], item);
    assert_eq!(union.span.start, initial_span.start);
    assert_eq!(union.span.end, new_span.end);
}

#[test]
fn test_push_multiple_items() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal {
        pub span: Span,
    }

    let initial_span = Span {
        start: Position { offset: 5, line: 1, column: 6 },
        end: Position { offset: 10, line: 1, column: 11 },
    };
    
    let mut union = ClassSetUnion {
        span: initial_span,
        items: vec![],
    };

    let item1 = ClassSetItem::Literal(Literal {
        span: Span {
            start: Position { offset: 0, line: 1, column: 1 },
            end: Position { offset: 2, line: 1, column: 3 },
        },
    });

    let item2 = ClassSetItem::Literal(Literal {
        span: Span {
            start: Position { offset: 3, line: 1, column: 4 },
            end: Position { offset: 5, line: 1, column: 6 },
        },
    });

    union.push(item1.clone());
    union.push(item2.clone());

    assert_eq!(union.items.len(), 2);
    assert_eq!(union.items[0], item1);
    assert_eq!(union.items[1], item2);
    assert_eq!(union.span.start, item1.span().start);
    assert_eq!(union.span.end, item2.span().end);
}

