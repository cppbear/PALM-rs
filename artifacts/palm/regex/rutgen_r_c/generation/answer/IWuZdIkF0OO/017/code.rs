// Answer 0

fn test_suffixes_repetition_range_exactly() {
    #[derive(Clone)]
    struct DummyAnchor;
    
    impl hir::Anchor for DummyAnchor {}
    
    let mut lits = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)),
            greedy: true,
            hir: Box::new(Hir {
                kind: HirKind::Literal(hir::Literal::Byte(b'a')),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

fn test_suffixes_repetition_range_at_least() {
    #[derive(Clone)]
    struct DummyAnchor;
    
    impl hir::Anchor for DummyAnchor {}

    let mut lits = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)),
            greedy: true,
            hir: Box::new(Hir {
                kind: HirKind::Literal(hir::Literal::Byte(b'b')),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&expr, &mut lits);
    
    assert!(lits.is_empty());
}

fn test_suffixes_repetition_range_bounded() {
    #[derive(Clone)]
    struct DummyAnchor;

    impl hir::Anchor for DummyAnchor {}

    let mut lits = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 3)),
            greedy: true,
            hir: Box::new(Hir {
                kind: HirKind::Literal(hir::Literal::Unicode('c')),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

