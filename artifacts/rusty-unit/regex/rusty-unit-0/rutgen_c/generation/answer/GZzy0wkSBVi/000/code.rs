// Answer 0

#[test]
fn test_prefixes_empty_hir() {
    struct MockHir {
        kind: HirKind,
        info: HirInfo,
    }

    let empty_hir = MockHir {
        kind: HirKind::Concat(vec![]),
        info: HirInfo::default(),
    };

    let result = prefixes(&empty_hir);

    assert_eq!(result.literals().len(), 0);
}

#[test]
fn test_prefixes_non_empty_hir() {
    struct MockHir {
        kind: HirKind,
        info: HirInfo,
    }

    let non_empty_hir = MockHir {
        kind: HirKind::Concat(vec![Hir::from_literal(Literal::Unicode('a'))]),
        info: HirInfo::default(),
    };

    let result = prefixes(&non_empty_hir);

    assert!(!result.is_empty());
    assert_eq!(result.literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_prefixes_hir_with_multiple_literals() {
    struct MockHir {
        kind: HirKind,
        info: HirInfo,
    }

    let multiple_literals_hir = MockHir {
        kind: HirKind::Concat(vec![
            Hir::from_literal(Literal::Unicode('a')),
            Hir::from_literal(Literal::Unicode('b')),
        ]),
        info: HirInfo::default(),
    };

    let result = prefixes(&multiple_literals_hir);

    assert_eq!(result.literals().len(), 2);
    assert_eq!(result.literals()[0], Literal::Unicode('a'));
    assert_eq!(result.literals()[1], Literal::Unicode('b'));
}

