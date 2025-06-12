// Answer 0

#[test]
fn test_class_exceeds_limits_greater_than_limit_class() {
    struct Literal {
        length: usize,
        cut: bool,
    }

    impl Literal {
        fn is_cut(&self) -> bool {
            self.cut
        }

        fn len(&self) -> usize {
            self.length
        }
    }

    struct Class {
        limit_class: usize,
        limit_size: usize,
        lits: Vec<Literal>,
    }

    impl Class {
        fn class_exceeds_limits(&self, size: usize) -> bool {
            if size > self.limit_class {
                return true;
            }
            let new_byte_count =
                if self.lits.is_empty() {
                    size
                } else {
                    self.lits
                        .iter()
                        .fold(0, |accum, lit| {
                            accum + if lit.is_cut() {
                                0
                            } else {
                                (lit.len() + 1) * size
                            }
                        })
                };
            new_byte_count > self.limit_size
        }
    }

    let size = 6; // greater than limit_class
    let limit_class = 5; // less than size
    let limit_size = 50; // arbitrary limit size for testing
    let literals = vec![
        Literal { length: 2, cut: false },
        Literal { length: 3, cut: false },
    ];

    let class = Class {
        limit_class,
        limit_size,
        lits: literals,
    };

    assert!(class.class_exceeds_limits(size));
}

