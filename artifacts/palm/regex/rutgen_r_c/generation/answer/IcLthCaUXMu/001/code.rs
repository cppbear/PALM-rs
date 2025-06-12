// Answer 0

#[test]
fn test_class_exceeds_limits_size_greater_than_limit_class() {
    struct TestLiteral {
        cut: bool,
        length: usize,
    }

    impl TestLiteral {
        fn is_cut(&self) -> bool {
            self.cut
        }

        fn len(&self) -> usize {
            self.length
        }
    }

    let limit_class = 5;
    let limit_size = 100;

    let literals = vec![
        TestLiteral { cut: false, length: 10 },
        TestLiteral { cut: false, length: 15 },
    ];

    let mut literals_struct = Literals {
        lits: literals.into_iter().map(|lit| Literal::Unicode('a')).collect(),
        limit_size,
        limit_class,
    };

    // Size that exceeds limit_class
    let size = 6;
    assert!(literals_struct.class_exceeds_limits(size));
}

#[test]
fn test_class_exceeds_limits_empty_literals() {
    let limit_class = 3;
    let limit_size = 50;

    let literals_struct = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class,
    };

    // Size that exceeds limit_class
    let size = 4;
    assert!(literals_struct.class_exceeds_limits(size));
}

#[test]
fn test_class_exceeds_limits_exact_limit_class() {
    let limit_class = 5;
    let limit_size = 50;

    let literals = vec![
        Literal::Unicode('a'),
        Literal::Unicode('b'),
        Literal::Unicode('c'),
    ];

    let literals_struct = Literals {
        lits: literals,
        limit_size,
        limit_class,
    };

    // Size equal to limit_class
    let size = 5;
    assert!(!literals_struct.class_exceeds_limits(size));
}

