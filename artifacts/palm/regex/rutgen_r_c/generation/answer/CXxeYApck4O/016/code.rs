// Answer 0

#[test]
fn test_prefixes_repetition_at_least() {
    use hir::{self, Hir, Repetition, RepetitionKind};

    let mut lits = Literals::empty();
    let lit_a = Literal::Unicode('a');
    let lit_b = Literal::Unicode('b');

    // Create a repetition of the literal 'a' with a range of at least 1
    let repetition = Repetition {
        kind: RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(1))),
        greedy: true,
        hir: Box::new(Hir::literal(lit_a.clone())),
    };

    let expr = Hir::Repetition(repetition);

    // Call the prefixes function with the constructed expression
    prefixes(&expr, &mut lits);

    // Check that the literals contain the expected results
    assert!(lits.any_complete());
    assert_eq!(lits.literals().len(), 1);
    assert_eq!(lits.literals()[0], lit_a); // Check that 'a' was added due to the repetition
    
    // Add more repetitions
    let repetition_b = Repetition {
        kind: RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(1))),
        greedy: true,
        hir: Box::new(Hir::literal(lit_b.clone())),
    };

    // This will repeat 'b'
    let expr_b = Hir::Repetition(repetition_b);
    prefixes(&expr_b, &mut lits);

    // Check that literals now contain both 'a' and 'b'
    assert_eq!(lits.literals().len(), 2);
    assert_eq!(lits.literals()[1], lit_b); // Check for 'b' as well
}

