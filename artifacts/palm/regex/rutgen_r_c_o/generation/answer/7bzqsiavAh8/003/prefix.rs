// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    let expr = Hir::literal(Literal::new("test"));
    let mut lits = Literals::empty();
    repeat_range_literals(&expr, 0, None, true, &mut lits, |h, l| {
        // Test function called
    });
}

#[test]
fn test_repeat_range_literals_min_greater_than_zero() {
    let expr = Hir::literal(Literal::new("test"));
    let mut lits = Literals {
        lits: vec![],
        limit_size: 1,
        limit_class: 1,
    };
    repeat_range_literals(&expr, 1, Some(2), true, &mut lits, |h, l| {
        // Test function called
    });
}

#[test]
fn test_repeat_range_literals_n_less_than_min() {
    let expr = Hir::literal(Literal::new("test"));
    let mut lits = Literals {
        lits: vec![],
        limit_size: 1,
        limit_class: 1,
    };
    repeat_range_literals(&expr, 2, Some(3), true, &mut lits, |h, l| {
        // Test function called
    });
}

#[test]
fn test_repeat_range_literals_max_not_less_than_min() {
    let expr = Hir::literal(Literal::new("test"));
    let mut lits = Literals {
        lits: vec![],
        limit_size: 2,
        limit_class: 1,
    };
    repeat_range_literals(&expr, 3, Some(3), true, &mut lits, |h, l| {
        // Test function called
    });
}

