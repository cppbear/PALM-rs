// Answer 0

#[test]
fn test_suffixes_empty_hir() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Empty,
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&expr);
    assert!(result.is_empty());
}

#[test]
fn test_suffixes_non_empty_hir() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Concat(vec![
            hir::Hir::literal(hir::Literal::Unicode('a')),
            hir::Hir::literal(hir::Literal::Unicode('b')),
        ]),
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&expr);
    assert!(!result.is_empty());
}

#[test]
fn test_suffixes_prefixes() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Union(vec![
            hir::Hir::literal(hir::Literal::Unicode('x')),
            hir::Hir::literal(hir::Literal::Unicode('y')),
        ]),
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&expr);
    assert_eq!(result.literals().len(), 2);
}

#[test]
fn test_suffixes_with_cut() {
    struct MockHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let expr = MockHir {
        kind: hir::HirKind::Concat(vec![
            hir::Hir::literal(hir::Literal::Unicode('c')),
            hir::Hir::literal(hir::Literal::Unicode('d')),
        ]),
        info: hir::HirInfo::default(),
    };

    let mut result = suffixes(&expr);
    result.cut();
    assert!(result.literals().iter().all(|lit| lit.cut));
}

