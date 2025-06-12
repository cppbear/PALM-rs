// Answer 0

#[test]
fn test_suffixes_empty_hir() {
    struct EmptyHir {
        kind: HirKind,
        info: hir::HirInfo,
    }

    let empty_hir = EmptyHir {
        kind: HirKind::Group(Vec::new()),
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&empty_hir);
    assert_eq!(result.is_empty(), true);
}

#[test]
fn test_suffixes_non_empty_hir() {
    struct NonEmptyHir {
        kind: HirKind,
        info: hir::HirInfo,
    }

    let literals = vec![
        Literal::Unicode('a'),
        Literal::Unicode('b'),
    ];
    
    let non_empty_hir = NonEmptyHir {
        kind: HirKind::Union(literals),
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&non_empty_hir);
    assert!(result.is_empty() == false);
    assert_eq!(result.literals().len(), 2); // assuming it results in literals count
}

#[test]
fn test_suffixes_contains_empty() {
    struct HIRWithEmptySuffix {
        kind: HirKind,
        info: hir::HirInfo,
    }

    let literals = vec![
        Literal::Unicode('c'),
        Literal::Unicode('\0'), // adding an empty literal
    ];

    let hir_with_empty = HIRWithEmptySuffix {
        kind: HirKind::Concatenation(literals),
        info: hir::HirInfo::default(),
    };

    let result = suffixes(&hir_with_empty);
    assert!(result.contains_empty());
} 

#[test]
fn test_suffixes_limitations() {
    struct LimitedHir {
        kind: HirKind,
        info: hir::HirInfo,
    }

    let literals = vec![
        Literal::Unicode('x'),
        Literal::Byte(1),
        Literal::Unicode('y'),
    ];

    let limited_hir = LimitedHir {
        kind: HirKind::Union(literals),
        info: hir::HirInfo::default(),
    };

    let mut result = suffixes(&limited_hir);
    result.set_limit_class(2);
    assert_eq!(result.limit_class(), 2);
    assert!(result.cross_product(&result));
}

