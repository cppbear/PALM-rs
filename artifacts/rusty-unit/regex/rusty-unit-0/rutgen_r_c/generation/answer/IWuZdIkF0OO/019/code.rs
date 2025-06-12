// Answer 0

fn test_suffixes_repetition_zero_or_more() {
    use hir::{Hir, HirKind, Repetition, RepetitionKind, Class, Literal};

    struct TestHir {
        kind: HirKind,
    }

    let mut lits = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    let repetition_expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir {
                kind: HirKind::Literal(Literal::Unicode('a')),
                info: Default::default(),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&repetition_expr, &mut lits);

    assert!(!lits.is_empty(), "Literals should not be empty after processing a zero or more repetition");
    assert!(lits.limit_size() >= 1, "Limit size should allow for at least one literal");
}

fn test_suffixes_repetition_zero_or_more_with_non_empty() {
    use hir::{Hir, HirKind, Repetition, RepetitionKind, Class, Literal};

    struct TestHir {
        kind: HirKind,
    }

    let mut lits = Literals {
        lits: vec![Literal::new(vec![97]), Literal::new(vec![98])], // a and b
        limit_size: 10,
        limit_class: 5,
    };

    let repetition_expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir {
                kind: HirKind::Literal(Literal::Unicode('c')),
                info: Default::default(),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&repetition_expr, &mut lits);

    assert!(!lits.is_empty(), "Literals should not be empty after processing a zero or more repetition");
    assert!(lits.literals().len() > 2, "Literals length should be greater than the original after suffixes");
}

fn test_suffixes_repetition_zero_or_more_empty_lits() {
    use hir::{Hir, HirKind, Repetition, RepetitionKind, Class, Literal};

    let mut lits = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    let repetition_expr = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir {
                kind: HirKind::Literal(Literal::Unicode('d')),
                info: Default::default(),
            }),
        }),
        info: Default::default(),
    };

    suffixes(&repetition_expr, &mut lits);

    assert!(lits.is_empty(), "Literals should be empty when starting from empty Literals");
}

