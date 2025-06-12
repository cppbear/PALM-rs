// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    struct DummyLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, limit_class: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class,
            }
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn contains_empty(&self) -> bool {
            self.lits.iter().any(|lit| lit.is_empty())
        }

        fn cut(&mut self) {
            self.lits.clear();
        }
    }

    let e = Hir::literal(Literal::from('a'));
    let min = 0;
    let max = None;
    let greedy = true;
    let mut lits = DummyLiterals::new(5, 0);

    repeat_range_literals(&e, min, max, greedy, &mut lits, |hir, lits| {
        lits.lits.push(hir.clone());
    });

    assert_eq!(lits.lits.len(), 1);
}

#[test]
fn test_repeat_range_literals_min_nonzero() {
    struct DummyLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, limit_class: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class,
            }
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn contains_empty(&self) -> bool {
            self.lits.iter().any(|lit| lit.is_empty())
        }

        fn cut(&mut self) {
            self.lits.clear();
        }
    }

    let e = Hir::literal(Literal::from('a'));
    let min = 2;
    let max = None;
    let greedy = true;
    let mut lits = DummyLiterals::new(5, 0);

    repeat_range_literals(&e, min, max, greedy, &mut lits, |hir, lits| {
        lits.lits.push(hir.clone());
    });

    assert_eq!(lits.lits.len(), 2);
}

#[test]
fn test_repeat_range_literals_min_greater_than_limit() {
    struct DummyLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, limit_class: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class,
            }
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn contains_empty(&self) -> bool {
            self.lits.iter().any(|lit| lit.is_empty())
        }

        fn cut(&mut self) {
            self.lits.clear();
        }
    }

    let e = Hir::literal(Literal::from('a'));
    let min = 10;
    let max = None;
    let greedy = true;
    let mut lits = DummyLiterals::new(5, 0);

    repeat_range_literals(&e, min, max, greedy, &mut lits, |hir, lits| {
        lits.lits.push(hir.clone());
    });

    assert_eq!(lits.lits.len(), 0);
}

#[test]
fn test_repeat_range_literals_with_cutting() {
    struct DummyLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, limit_class: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class,
            }
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn contains_empty(&self) -> bool {
            self.lits.iter().any(|lit| lit.is_empty())
        }

        fn cut(&mut self) {
            self.lits.clear();
        }
    }

    let e = Hir::literal(Literal::from('a'));
    let min = 1;
    let max = Some(3);
    let greedy = false;
    let mut lits = DummyLiterals::new(3, 0);

    repeat_range_literals(&e, min, max, greedy, &mut lits, |hir, lits| {
        lits.lits.push(hir.clone());
    });

    assert!(lits.lits.len() <= 3);
}

