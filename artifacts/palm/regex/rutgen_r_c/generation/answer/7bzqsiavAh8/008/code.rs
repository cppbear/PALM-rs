// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    struct MockHir {
        is_empty: bool,
    }

    impl MockHir {
        fn empty() -> Self {
            MockHir { is_empty: true }
        }

        fn clone(&self) -> Self {
            Self { is_empty: self.is_empty }
        }
    }

    impl Hir {
        fn repetition(rep: Repetition) -> Hir {
            Hir {
                kind: HirKind::Repetition(rep),
                info: HirInfo::new(),
            }
        }
    }

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 1,
    };

    let e = Hir::literal(MockHir::empty());
    
    repeat_range_literals(&e, 0, None, true, &mut literals, |hir, lits| {
        lits.add(Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(hir.clone()),
        }));
    });

    assert_eq!(literals.lits.len(), 1);
}

#[test]
fn test_repeat_range_literals_min_greater_than_zero() {
    struct MockHir {
        is_empty: bool,
    }

    impl MockHir {
        fn empty() -> Self {
            MockHir { is_empty: false }
        }

        fn clone(&self) -> Self {
            Self { is_empty: self.is_empty }
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }
    }

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 1,
    };

    let e = Hir::literal(MockHir::empty());

    repeat_range_literals(&e, 1, Some(2), true, &mut literals, |hir, lits| {
        lits.add(hir.clone());
    });

    assert_eq!(literals.lits.len(), 1);
}

#[test]
fn test_repeat_range_literals_min_less_than_max() {
    struct MockHir {
        is_empty: bool,
    }

    impl MockHir {
        fn empty() -> Self {
            MockHir { is_empty: false }
        }

        fn clone(&self) -> Self {
            Self { is_empty: self.is_empty }
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }
    }

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 1,
    };

    let e = Hir::literal(MockHir::empty());

    repeat_range_literals(&e, 1, Some(3), true, &mut literals, |hir, lits| {
        lits.add(hir.clone());
    });

    assert_eq!(literals.lits.len(), 1);
}

