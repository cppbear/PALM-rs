// Answer 0

#[test]
fn test_suffixes_with_repetition_range_at_least() {
    use hir::{self, Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};

    let mut literals = Literals::empty();
    
    // Construct a Repetition with AtLeast(2) kind
    let repetitions = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(2)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    };

    let hir = Hir::Repetition(repetitions);
    
    // Call the suffixes function
    suffixes(&hir, &mut literals);
    
    // Check the manipulated literals after suffixes
    assert!(!literals.is_empty());
    assert_eq!(literals.limit_size(), 0); // Assuming you want to test the default size
}

#[test]
fn test_suffixes_with_repetition_range_bounded() {
    use hir::{self, Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};

    let mut literals = Literals::empty();
    
    // Construct a Repetition with Bounded(1, 3) kind
    let repetitions = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 3)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    };

    let hir = Hir::Repetition(repetitions);
    
    // Call the suffixes function
    suffixes(&hir, &mut literals);
    
    // Check the manipulated literals after suffixes
    assert!(!literals.is_empty());
    assert_eq!(literals.limit_size(), 0); // Assuming you want to test the default size
}

