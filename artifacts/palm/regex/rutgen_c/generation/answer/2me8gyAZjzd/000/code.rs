// Answer 0

#[test]
fn test_limit_size() {
    struct TestLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl Literals {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class: 0,
            }
        }
    }

    let literals = TestLiterals::new(10);
    assert_eq!(literals.limit_size(), 10);
}

#[test]
fn test_limit_size_zero() {
    struct TestLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl Literals {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class: 0,
            }
        }
    }

    let literals = TestLiterals::new(0);
    assert_eq!(literals.limit_size(), 0);
}

#[test]
fn test_limit_size_large_value() {
    struct TestLiterals {
        lits: Vec<Literal>,
        limit_size: usize,
        limit_class: usize,
    }

    impl Literals {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
                limit_class: 0,
            }
        }
    }

    let literals = TestLiterals::new(usize::MAX);
    assert_eq!(literals.limit_size(), usize::MAX);
}

