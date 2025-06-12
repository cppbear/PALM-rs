// Answer 0

#[test]
fn test_repeat_zero_or_more_literals() {
    #[derive(Clone)]
    struct TestHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let mut lits = {
        let mut literals = Literals::empty();
        literals.set_limit_size(100);
        literals
    };

    let lits2 = {
        let lit = Literal::new(vec![b'a', b'b']);
        lits.add(lit.clone());
        Literals { lits: vec![lit], limit_size: 100, limit_class: 1 } 
    };

    let lits3 = {
        let lit = Literal::new(vec![b'c']);
        let mut literals = Literals::empty();
        literals.set_limit_size(50); // Constraint: half of lits.limit_size()
        literals.add(lit);
        literals
    };

    // This setup ensures that lits3 is not empty and lits2 will succeed in its cross_product check
    let expr = TestHir { kind: hir::HirKind::Empty, info: hir::HirInfo::default() };

    // Simulation of the function's behavior
    let mut f = |e: &Hir, literals: &mut Literals| {
        literals.add(lit.clone()); // "f" modifies lits3 with the same literal to ensure it's not empty
    };

    // Set up conditions that would lead to union failing
    lits.union(lits2.clone());

    // Execute the function
    repeat_zero_or_more_literals(&expr, &mut lits, f);

    // Assert that the state of lits reflects that the cross product and union conditions were not met
    assert!(lits2.is_empty() == false);
    assert!(lits.is_empty() == false);
}

#[test]
fn test_repeat_zero_or_more_literals_failure_conditions() {
    #[derive(Clone)]
    struct TestHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }

    let mut lits = {
        let mut literals = Literals::empty();
        literals.set_limit_size(20); // Limit size set to 20
        literals
    };

    // Create an empty `lits2`, ensuring `lits.union(lits2)` fails
    let lits2 = {
        Literals { lits: vec![], limit_size: 20, limit_class: 1 }
    };

    // Create `lits3` that is non-empty
    let lits3 = {
        let lit = Literal::new(vec![b'x', b'y']);
        let mut literals = Literals::empty();
        literals.set_limit_size(10); // Constraint, limit size reduced
        literals.add(lit);
        literals
    };

    // Ensure the setup
    let expr = TestHir { kind: hir::HirKind::Empty, info: hir::HirInfo::default() };

    // Simulate the function's behavior
    let mut f = |e: &Hir, literals: &mut Literals| {
        literals.add(Literal::empty()); // f adds an empty Literal
    };

    // Execute the function
    repeat_zero_or_more_literals(&expr, &mut lits, f);

    // Assertions to check the conditions
    assert!(lits.is_empty() == true); // The cut should have been triggered
}



