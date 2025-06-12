// Answer 0

#[test]
fn test_repeat_range_literals_min_non_zero() {
    struct TestHir;
    impl TestHir {
        fn new_lit() -> Hir {
            Hir::literal(Literal::from_byte(b'a')) // Example literal
        }
    }

    let mut lits = Literals {
        lits: vec![],
        limit_size: 5,
        limit_class: 1,
    };

    let h = TestHir::new_lit();
    let mut collected = vec![];

    let f = |expr: &Hir, literals: &mut Literals| {
        collected.push(expr.clone());
        literals.add(Literal::from_byte(b'a')); // Simulate adding to literals
    };

    repeat_range_literals(&h, 3, None, true, &mut lits, f);
    
    assert_eq!(collected.len(), 3); // Expecting 3 repetitions to be collected
    assert_eq!(lits.lits.len(), 1); // One literal added in the callback
}

#[test]
fn test_repeat_range_literals_max_exceeding_limit() {
    struct TestHir;
    impl TestHir {
        fn new_lit() -> Hir {
            Hir::literal(Literal::from_byte(b'b')) // Example literal
        }
    }

    let mut lits = Literals {
        lits: vec![],
        limit_size: 2, // Limit is set to 2
        limit_class: 1,
    };

    let h = TestHir::new_lit();
    let mut collected = vec![];

    let f = |expr: &Hir, literals: &mut Literals| {
        collected.push(expr.clone());
        literals.add(Literal::from_byte(b'b')); // Simulate adding to literals
    };

    repeat_range_literals(&h, 3, Some(5), true, &mut lits, f);

    assert_eq!(collected.len(), 2); // Only limited to 2 due to limit_size
    assert_eq!(lits.lits.len(), 1); // One literal added in the callback
}

#[test]
fn test_repeat_range_literals_min_less_than_max() {
    struct TestHir;
    impl TestHir {
        fn new_lit() -> Hir {
            Hir::literal(Literal::from_byte(b'c')) // Example literal
        }
    }

    let mut lits = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let h = TestHir::new_lit();
    let mut collected = vec![];

    let f = |expr: &Hir, literals: &mut Literals| {
        collected.push(expr.clone());
        literals.add(Literal::from_byte(b'c')); // Simulate adding to literals
    };

    repeat_range_literals(&h, 2, Some(4), true, &mut lits, f);

    assert_eq!(collected.len(), 2); // Expecting 2 repetitions to be collected
    assert_eq!(lits.lits.len(), 1); // One literal added in the callback
}

#[test]
#[should_panic]
fn test_repeat_range_literals_panic_if_limit_exceeds_min() {
    struct TestHir;
    impl TestHir {
        fn new_lit() -> Hir {
            Hir::literal(Literal::from_byte(b'd')) // Example literal
        }
    }

    let mut lits = Literals {
        lits: vec![],
        limit_size: 1, // Limit set to 1
        limit_class: 1,
    };

    let h = TestHir::new_lit();

    let f = |_: &Hir, _: &mut Literals| {
        panic!("Panic during repetitive addition");
    };

    repeat_range_literals(&h, 5, None, true, &mut lits, f); // This should trigger panic
}

