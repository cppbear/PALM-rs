// Answer 0

#[test]
fn test_class_set_range_valid() {
    let start_literal = Literal {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let end_literal = Literal {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        kind: LiteralKind::Unicode('z'),
        c: 'z',
    };
    let class_set_range = ClassSetRange {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        start: start_literal,
        end: end_literal,
    };
    
    assert!(class_set_range.is_valid());
}

#[test]
fn test_class_set_range_invalid() {
    let start_literal = Literal {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        kind: LiteralKind::Unicode('z'),
        c: 'z',
    };
    let end_literal = Literal {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let class_set_range = ClassSetRange {
        span: Span { start: Position { /* initialization */ }, end: Position { /* initialization */ } },
        start: start_literal,
        end: end_literal,
    };

    assert!(!class_set_range.is_valid());
}

