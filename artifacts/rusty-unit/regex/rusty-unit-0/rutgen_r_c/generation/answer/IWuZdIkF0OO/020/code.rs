// Answer 0

#[test]
fn test_suffixes_zero_or_one_repetition() {
    // Construct necessary structs for the test
    let literal_a = hir::Literal::Unicode('a');
    let repetition_expr = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(hir::Hir::literal(literal_a.clone())),
    };
    let repetitions_hir = hir::Hir::Repetition(repetition_expr);

    // Initialize Literals
    let mut literals = Literals::empty();

    // Call the function under test
    suffixes(&repetitions_hir, &mut literals);

    // Assert expected behavior
    assert!(literals.any_complete());
    assert!(!literals.is_empty());
}

#[test]
fn test_suffixes_zero_or_one_repetition_empty() {
    // Construct necessary structs for the test
    let repetition_expr = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: false,
        hir: Box::new(hir::Hir::empty()),
    };
    let repetitions_hir = hir::Hir::Repetition(repetition_expr);

    // Initialize Literals
    let mut literals = Literals::empty();

    // Call the function under test
    suffixes(&repetitions_hir, &mut literals);

    // Assert expected behavior
    assert!(literals.is_empty());
}

#[test]
#[should_panic]
fn test_suffixes_zero_or_one_repetition_cut() {
    // Construct necessary structs for the test
    let literal_a = hir::Literal::Unicode('a');
    let repetition_expr = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(hir::Hir::literal(literal_a)),
    };
    let repetitions_hir = hir::Hir::Repetition(repetition_expr);

    // Initialize Literals with a cut
    let mut literals = Literals::empty();
    literals.cut();

    // Call the function under test
    suffixes(&repetitions_hir, &mut literals); // This should trigger a panic due to `lits.cut()`
}

