// Answer 0

#[test]
fn test_any_complete_with_non_cut_literals() {
    struct TestLiteral {
        cut: bool,
    }

    impl TestLiteral {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    let literals = Literals {
        lits: vec![
            TestLiteral { cut: false },
            TestLiteral { cut: false },
        ],
        limit_size: 10,
        limit_class: 10,
    };

    assert_eq!(literals.any_complete(), true);
}

#[test]
fn test_any_complete_with_all_cut_literals() {
    struct TestLiteral {
        cut: bool,
    }

    impl TestLiteral {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    let literals = Literals {
        lits: vec![
            TestLiteral { cut: true },
            TestLiteral { cut: true },
        ],
        limit_size: 10,
        limit_class: 10,
    };

    assert_eq!(literals.any_complete(), false);
}

#[test]
fn test_any_complete_with_mixed_cut_literals() {
    struct TestLiteral {
        cut: bool,
    }

    impl TestLiteral {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    let literals = Literals {
        lits: vec![
            TestLiteral { cut: true },
            TestLiteral { cut: false },
        ],
        limit_size: 10,
        limit_class: 10,
    };

    assert_eq!(literals.any_complete(), true);
}

#[test]
fn test_any_complete_with_empty_literals() {
    struct TestLiteral {
        cut: bool,
    }

    impl TestLiteral {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    let literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    assert_eq!(literals.any_complete(), false);
}

