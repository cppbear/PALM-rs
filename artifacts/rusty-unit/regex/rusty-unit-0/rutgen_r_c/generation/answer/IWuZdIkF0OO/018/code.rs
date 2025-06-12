// Answer 0

#[test]
fn test_suffixes_repetition_one_or_more() {
    use hir::{self, HirKind, Repetition, RepetitionKind};

    // Create a literal character
    let lit = hir::Literal::Unicode('a');

    // Create a HIR from the literal
    let hir = Hir::literal(lit.clone());

    // Create a repetition that matches the literal one or more times
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(hir),
    };

    // Create the HIR expression for the repetition
    let expr = Hir::repetition(repetition);

    // Create an empty Literals object
    let mut lits = Literals::empty();

    // Call the function under test
    suffixes(&expr, &mut lits);

    // Check the results in lits after the function call
    assert!(!lits.is_empty());
    assert_eq!(lits.limit_size(), 0); // Adjust this based on expected limits
}

#[test]
fn test_suffixes_repetition_one_or_more_with_empty_literal() {
    use hir::{self, HirKind, Repetition, RepetitionKind};

    // Create an empty literal
    let empty_lit = hir::Literal::empty();

    // Create a HIR from the empty literal
    let hir_empty = Hir::literal(empty_lit.clone());

    // Create a repetition that matches the empty literal one or more times
    let repetition_empty = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(hir_empty),
    };

    // Create the HIR expression for the repetition
    let expr_empty = Hir::repetition(repetition_empty);

    // Create an empty Literals object
    let mut lits_empty = Literals::empty();

    // Call the function under test
    suffixes(&expr_empty, &mut lits_empty);

    // Check the results in lits after the function call
    assert!(lits_empty.is_empty()); // Expect it to be empty since we started with an empty literal
}

#[test]
fn test_suffixes_repetition_one_or_more_with_multiple_literals() {
    use hir::{self, HirKind, Repetition, RepetitionKind};

    // Create multiple literals
    let lit_a = hir::Literal::Unicode('a');
    let lit_b = hir::Literal::Unicode('b');

    // Create HIR expressions from literals
    let hir_a = Hir::literal(lit_a);
    let hir_b = Hir::literal(lit_b);

    // Create a repetition that matches 'a' or 'b' one or more times
    let repetition_multi = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::alternation(vec![hir_a, hir_b])),
    };

    // Create the HIR expression for the repetition
    let expr_multi = Hir::repetition(repetition_multi);

    // Create an empty Literals object
    let mut lits_multi = Literals::empty();

    // Call the function under test
    suffixes(&expr_multi, &mut lits_multi);

    // Check the results in lits after the function call
    assert!(!lits_multi.is_empty());
    assert!(lits_multi.literals().iter().any(|lit| {
        if let Literal::Unicode(c) = lit {
            *c == 'a' || *c == 'b'
        } else {
            false
        }
    })); // Expect it to contain the literals 'a' or 'b'
}

#[test]
fn test_suffixes_repetition_zero_or_more() {
    use hir::{self, HirKind, Repetition, RepetitionKind};

    // Create a literal character
    let lit = hir::Literal::Unicode('x');

    // Create HIR from the literal
    let hir = Hir::literal(lit);

    // Create a repetition that matches the literal zero or more times
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(hir),
    };

    // Create the HIR expression for the repetition
    let expr = Hir::repetition(repetition);

    // Create an empty Literals object
    let mut lits = Literals::empty();

    // Call the function under test
    suffixes(&expr, &mut lits);

    // Check the results in lits after the function call
    assert!(lits.is_empty()); // Expect to have no literals as we're looking for suffixes
}

