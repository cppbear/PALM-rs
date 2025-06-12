// Answer 0

#[test]
fn test_span_literal() {
    let class_set = ClassSet::Item(ClassSetItem::Literal(Literal {
        span: Span {
            start: Position { byte: 0 },
            end: Position { byte: 1 },
        },
    }));
    class_set.span();
}

#[test]
fn test_span_empty() {
    let class_set = ClassSet::Item(ClassSetItem::Empty(Span {
        start: Position { byte: 0 },
        end: Position { byte: 1 },
    }));
    class_set.span();
}

#[test]
fn test_span_range() {
    let class_set = ClassSet::Item(ClassSetItem::Range(ClassSetRange {
        start: 'a',
        end: 'z',
        span: Span {
            start: Position { byte: 1 },
            end: Position { byte: 5 },
        },
    }));
    class_set.span();
}

#[test]
fn test_span_ascii() {
    let class_set = ClassSet::Item(ClassSetItem::Ascii(ClassAscii {
        span: Span {
            start: Position { byte: 2 },
            end: Position { byte: 6 },
        },
    }));
    class_set.span();
}

#[test]
fn test_span_unicode() {
    let class_set = ClassSet::Item(ClassSetItem::Unicode(ClassUnicode {
        span: Span {
            start: Position { byte: 3 },
            end: Position { byte: 7 },
        },
    }));
    class_set.span();
}

#[test]
fn test_span_perl() {
    let class_set = ClassSet::Item(ClassSetItem::Perl(ClassPerl {
        span: Span {
            start: Position { byte: 4 },
            end: Position { byte: 8 },
        },
    }));
    class_set.span();
}

#[test]
fn test_span_bracketed() {
    let items = vec![
        ClassSetItem::Range(ClassSetRange {
            start: 'a',
            end: 'b',
            span: Span {
                start: Position { byte: 5 },
                end: Position { byte: 9 },
            },
        }),
    ];
    let class_set = ClassSet::Item(ClassSetItem::Bracketed(Box::new(ClassBracketed { items })));
    class_set.span();
}

#[test]
fn test_span_union() {
    let class_set = ClassSet::Item(ClassSetItem::Union(ClassSetUnion {
        items: vec![
            ClassSetItem::Literal(Literal {
                span: Span {
                    start: Position { byte: 6 },
                    end: Position { byte: 10 },
                },
            }),
        ],
    }));
    class_set.span();
}

