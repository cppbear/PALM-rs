// Answer 0

#[test]
fn test_alternate_literals_empty_input() {
    struct Hir;
    struct Literals {
        limit_size: usize,
        is_frozen: bool,
    }

    impl Literals {
        fn to_empty(&self) -> Self {
            Literals {
                limit_size: 0,
                is_frozen: false,
            }
        }
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        fn limit_size(&self) -> usize {
            self.limit_size
        }
        fn is_empty(&self) -> bool {
            self.limit_size == 0
        }
        fn union(&mut self, other: &Self) -> bool {
            if other.is_empty() {
                return false;
            }
            true
        }
        fn cross_product(&mut self, other: &Self) -> bool {
            true
        }
        fn cut(&mut self) {
            self.is_frozen = true;
        }
    }

    let lits = &mut Literals {
        limit_size: 10,
        is_frozen: false,
    };
    let es: Vec<Hir> = vec![];

    alternate_literals(&es, lits, |_, _| {});

    assert!(lits.is_frozen);
}

#[test]
fn test_alternate_literals_single_element() {
    struct Hir;
    struct Literals {
        limit_size: usize,
        is_frozen: bool,
    }

    impl Literals {
        fn to_empty(&self) -> Self {
            Literals {
                limit_size: 0,
                is_frozen: false,
            }
        }
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        fn limit_size(&self) -> usize {
            self.limit_size
        }
        fn is_empty(&self) -> bool {
            self.limit_size == 0
        }
        fn union(&mut self, other: &Self) -> bool {
            if other.is_empty() {
                return false;
            }
            true
        }
        fn cross_product(&mut self, other: &Self) -> bool {
            true
        }
        fn cut(&mut self) {
            self.is_frozen = true;
        }
    }

    let lits = &mut Literals {
        limit_size: 10,
        is_frozen: false,
    };
    let es: Vec<Hir> = vec![Hir];

    alternate_literals(&es, lits, |_, _| {});

    assert!(!lits.is_frozen);
}

#[test]
fn test_alternate_literals_multiple_elements() {
    struct Hir;
    struct Literals {
        limit_size: usize,
        is_frozen: bool,
    }

    impl Literals {
        fn to_empty(&self) -> Self {
            Literals {
                limit_size: 0,
                is_frozen: false,
            }
        }
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        fn limit_size(&self) -> usize {
            self.limit_size
        }
        fn is_empty(&self) -> bool {
            self.limit_size == 0
        }
        fn union(&mut self, other: &Self) -> bool {
            if other.is_empty() {
                return false;
            }
            true
        }
        fn cross_product(&mut self, other: &Self) -> bool {
            true
        }
        fn cut(&mut self) {
            self.is_frozen = true;
        }
    }

    let lits = &mut Literals {
        limit_size: 10,
        is_frozen: false,
    };
    let es: Vec<Hir> = vec![Hir, Hir, Hir];

    alternate_literals(&es, lits, |_, _| {});

    assert!(!lits.is_frozen);
}

#[test]
fn test_alternate_literals_union_failure() {
    struct Hir;
    struct Literals {
        limit_size: usize,
        is_frozen: bool,
    }

    impl Literals {
        fn to_empty(&self) -> Self {
            Literals {
                limit_size: 0,
                is_frozen: false,
            }
        }
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        fn limit_size(&self) -> usize {
            self.limit_size
        }
        fn is_empty(&self) -> bool {
            self.limit_size == 0
        }
        fn union(&mut self, other: &Self) -> bool {
            false
        }
        fn cross_product(&mut self, other: &Self) -> bool {
            true
        }
        fn cut(&mut self) {
            self.is_frozen = true;
        }
    }

    let lits = &mut Literals {
        limit_size: 10,
        is_frozen: false,
    };
    let es: Vec<Hir> = vec![Hir, Hir];

    alternate_literals(&es, lits, |_, _| {});

    assert!(lits.is_frozen);
}

