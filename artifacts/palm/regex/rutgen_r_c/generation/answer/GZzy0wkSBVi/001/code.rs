// Answer 0

#[test]
fn test_empty_prefixes() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Empty,
        info: hir::HirInfo::default(),
    };

    let lits = prefixes(&expr);
    assert_eq!(lits.is_empty(), true);
}

#[test]
fn test_single_character_prefixes() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Literal(hir::Literal::Unicode('a')),
        info: hir::HirInfo::default(),
    };

    let lits = prefixes(&expr);
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_multiple_character_prefixes() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Concatenation(vec![
            hir::Literal::Unicode('a'),
            hir::Literal::Unicode('b'),
        ]),
        info: hir::HirInfo::default(),
    };

    let lits = prefixes(&expr);
    assert_eq!(lits.literals().len(), 2);
    assert_eq!(lits.literals()[0], Literal::Unicode('a'));
    assert_eq!(lits.literals()[1], Literal::Unicode('b'));
}

#[test]
fn test_prefixes_with_empty_literal() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Alternation(vec![
            hir::Literal::Unicode('a'),
            hir::Literal::Empty,
        ]),
        info: hir::HirInfo::default(),
    };

    let lits = prefixes(&expr);
    assert_eq!(lits.contains_empty(), false);
}

#[test]
fn test_large_literal_prefixes() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Repeater(Box::new(hir::Literal::Unicode('a')), 1..=5),
        info: hir::HirInfo::default(),
    };

    let lits = prefixes(&expr);
    assert!(lits.literals().len() <= 10); // Checking against the limit_class
    assert!(lits.literals().iter().all(|lit| matches!(lit, Literal::Unicode('a'))));
}

