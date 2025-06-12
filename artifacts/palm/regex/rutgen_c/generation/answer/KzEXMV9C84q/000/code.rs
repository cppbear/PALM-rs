// Answer 0

#[test]
fn test_repeat_zero_or_more_literals_empty() {
    struct DummyHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let empty_lits = Literals::empty();
    let mut lits = empty_lits.clone();

    let expr = DummyHir {
        kind: hir::HirKind::Empty,
        info: hir::HirInfo::default(),
    };

    repeat_zero_or_more_literals(&expr, &mut lits, |_, _| {});

    assert!(lits.is_empty());
}

#[test]
fn test_repeat_zero_or_more_literals_non_empty() {
    struct DummyHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    let literal = Literal::new(vec![1, 2, 3, 4]);
    lits.add(literal);

    let expr = DummyHir {
        kind: hir::HirKind::Characters,
        info: hir::HirInfo::default(),
    };

    repeat_zero_or_more_literals(&expr, &mut lits, |_, lits3| {
        // This callback populates lits3
        let empty_literal = Literal::empty();
        lits3.add(empty_literal);
    });

    assert!(!lits.is_empty());
}

#[test]
fn test_repeat_zero_or_more_literals_cross_product_failure() {
    struct DummyHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let mut lits = Literals::empty();
    lits.set_limit_size(0); // Set limit to 0 to force a failure

    let expr = DummyHir {
        kind: hir::HirKind::Characters,
        info: hir::HirInfo::default(),
    };

    repeat_zero_or_more_literals(&expr, &mut lits, |_, lits3| {
        let literal = Literal::new(vec![1, 2, 3, 4]);
        lits3.add(literal);
    });

    assert!(lits.is_empty());
}

