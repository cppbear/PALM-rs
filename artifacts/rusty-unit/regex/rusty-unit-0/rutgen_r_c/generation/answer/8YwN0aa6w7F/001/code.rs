// Answer 0

#[test]
fn test_hirkind_has_subexprs_with_alternation() {
    struct DummyHir;

    let sub_exprs = vec![
        Hir {
            kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' }),
            info: HirInfo::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' }),
            info: HirInfo::default(),
        },
    ];

    let alternation = HirKind::Alternation(sub_exprs);
    assert!(alternation.has_subexprs());
}

