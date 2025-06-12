// Answer 0

#[test]
fn test_is_empty_variant_empty() {
    let hir_kind = HirKind::Empty;
    assert!(hir_kind.is_empty());
}

#[test]
fn test_is_empty_variant_literal() {
    let literal = Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let hir_kind = HirKind::Literal(literal);
    assert!(!hir_kind.is_empty());
}

#[test]
fn test_is_empty_variant_group() {
    let group = Group {
        span: Span { start: 0, end: 1 },
        kind: GroupKind::Capturing(0),
        ast: Box::new(Hir { kind: HirKind::Empty, info: HirInfo::default() }),
    };
    let hir_kind = HirKind::Group(group);
    assert!(!hir_kind.is_empty());
}

#[test]
fn test_is_empty_variant_concat() {
    let concat = HirKind::Concat(vec![Hir { kind: HirKind::Literal(Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Unicode('a'), c: 'a' }) }]);
    assert!(!concat.is_empty());
}

