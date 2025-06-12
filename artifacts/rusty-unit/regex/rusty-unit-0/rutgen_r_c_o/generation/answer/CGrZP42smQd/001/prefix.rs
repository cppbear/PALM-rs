// Answer 0

#[test]
fn test_push_empty_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 0,
                line: 1,
                column: 1
            },
            end: Position {
                offset: 0,
                line: 1,
                column: 1
            }
        },
        items: vec![],
    };
    let item = ClassSetItem::Empty(Span {
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
    });
    union.push(item);
}

#[test]
fn test_push_literal_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 0,
                line: 1,
                column: 1
            },
            end: Position {
                offset: 0,
                line: 1,
                column: 1
            }
        },
        items: vec![],
    };
    let item = ClassSetItem::Literal(Literal { 
        span: Span { 
            start: Position { 
                offset: 10, 
                line: 1, 
                column: 11 
            }, 
            end: Position { 
                offset: 15, 
                line: 1, 
                column: 16 
            } 
        } 
    });
    union.push(item);
}

#[test]
fn test_push_range_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 5,
                line: 1,
                column: 6
            },
            end: Position {
                offset: 5,
                line: 1,
                column: 6
            }
        },
        items: vec![],
    };
    let item = ClassSetItem::Range(ClassSetRange {
        span: Span {
            start: Position {
                offset: 100,
                line: 1,
                column: 101,
            },
            end: Position {
                offset: 200,
                line: 1,
                column: 201,
            },
        },
    });
    union.push(item);
}

#[test]
fn test_push_ascii_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 3,
                line: 1,
                column: 4
            },
            end: Position {
                offset: 3,
                line: 1,
                column: 4
            }
        },
        items: vec![],
    };
    let item = ClassSetItem::Ascii(ClassAscii {
        span: Span {
            start: Position {
                offset: 50,
                line: 1,
                column: 51,
            },
            end: Position {
                offset: 60,
                line: 1,
                column: 61,
            },
        },
    });
    union.push(item);
}

#[test]
fn test_push_bracketed_item() {
    let mut union = ClassSetUnion {
        span: Span {
            start: Position {
                offset: 1,
                line: 1,
                column: 2
            },
            end: Position {
                offset: 1,
                line: 1,
                column: 2
            }
        },
        items: vec![],
    };
    let item = ClassSetItem::Bracketed(Box::new(ClassBracketed {
        span: Span {
            start: Position {
                offset: 30,
                line: 1,
                column: 31,
            },
            end: Position {
                offset: 40,
                line: 1,
                column: 41,
            },
        },
        items: vec![],
    }));
    union.push(item);
}

