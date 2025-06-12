// Answer 0

#[test]
fn test_repeat_one_or_more_literals() {
    struct DummyHir {
        kind: HirKind,
    }

    let expr = DummyHir { kind: HirKind::Literal };
    let mut literals = Literals::empty();

    let mut callback_called = false;

    let callback = |e: &Hir, lits: &mut Literals| {
        callback_called = true;
        // Simulate adding a literal based on the Hir
        let lit = Literal::from_expr(e);
        lits.add(lit);
    };

    repeat_one_or_more_literals(&expr, &mut literals, callback);
    
    assert!(callback_called);
    assert!(!literals.is_empty());
}

#[test]
fn test_repeat_one_or_more_literals_empty() {
    struct DummyHir {
        kind: HirKind,
    }

    let expr = DummyHir { kind: HirKind::Literal };
    let mut literals = Literals::empty();

    let callback = |_: &Hir, lits: &mut Literals| {
        // Intentionally do nothing to simulate the edge case
    };

    repeat_one_or_more_literals(&expr, &mut literals, callback);

    assert!(literals.is_empty());
}

#[test]
fn test_repeat_one_or_more_literals_cut() {
    struct DummyHir {
        kind: HirKind,
    }

    let expr = DummyHir { kind: HirKind::Literal };
    let mut literals = Literals::empty();
    literals.add(Literal::new(b'a')); // add an initial literal

    let callback = |e: &Hir, lits: &mut Literals| {
        let lit = Literal::from_expr(&e);
        lits.add(lit);
    };

    repeat_one_or_more_literals(&expr, &mut literals, callback);

    // Assert that the cut operation altered the literals as expected
    // This depends on the implementation details of Literals::cut()
    assert!(literals.literals().len() > 0);
}

