// Answer 0

#[test]
fn test_class_exceeds_limits_with_empty_literals_and_size_zero() {
    let literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };
    assert!(!literals.class_exceeds_limits(0));
}

#[test]
fn test_class_exceeds_limits_with_empty_literals_and_size_exceeding_limit_class() {
    let literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };
    assert!(literals.class_exceeds_limits(6));
}

#[test]
fn test_class_exceeds_limits_with_non_empty_literals_exceeding_limit_size() {
    let literal = Literal {
        span: Span::new(0, 1),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let literals = Literals {
        lits: vec![literal],
        limit_size: 10,
        limit_class: 5,
    };
    assert!(literals.class_exceeds_limits(3));
}

#[test]
fn test_class_exceeds_limits_with_non_empty_literals_within_limits() {
    let literal = Literal {
        span: Span::new(0, 1),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let literals = Literals {
        lits: vec![literal],
        limit_size: 10,
        limit_class: 5,
    };
    assert!(!literals.class_exceeds_limits(2));
}

#[test]
fn test_class_exceeds_limits_with_non_empty_literals_cut() {
    let literal = Literal {
        span: Span::new(0, 1),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let cut_literal = Literal {
        span: Span::new(1, 2),
        kind: LiteralKind::Unicode('b'),
        c: 'b',
        cut: true,
    };
    let literals = Literals {
        lits: vec![literal, cut_literal],
        limit_size: 10,
        limit_class: 5,
    };
    assert!(!literals.class_exceeds_limits(3));
}

