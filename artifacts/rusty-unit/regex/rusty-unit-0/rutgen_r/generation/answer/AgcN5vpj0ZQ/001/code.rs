// Answer 0

#[test]
fn test_to_empty() {
    struct TestLiterals {
        limit_size: usize,
        limit_class: Option<char>,
    }

    impl TestLiterals {
        fn empty() -> Self {
            TestLiterals {
                limit_size: 0,
                limit_class: None,
            }
        }

        fn set_limit_size(mut self, size: usize) -> Self {
            self.limit_size = size;
            self
        }

        fn set_limit_class(mut self, class: Option<char>) -> Self {
            self.limit_class = class;
            self
        }
        
        pub fn to_empty(&self) -> Self {
            let mut lits = TestLiterals::empty();
            lits.set_limit_size(self.limit_size)
                .set_limit_class(self.limit_class);
            lits
        }
    }

    // Test case 1: Default limits
    let literals = TestLiterals {
        limit_size: 10,
        limit_class: Some('a'),
    };
    let empty_lits = literals.to_empty();
    assert_eq!(empty_lits.limit_size, 10);
    assert_eq!(empty_lits.limit_class, Some('a'));

    // Test case 2: Minimum size limit
    let literals_min = TestLiterals {
        limit_size: 0,
        limit_class: None,
    };
    let empty_lits_min = literals_min.to_empty();
    assert_eq!(empty_lits_min.limit_size, 0);
    assert_eq!(empty_lits_min.limit_class, None);

    // Test case 3: Maximum size limit
    let literals_max = TestLiterals {
        limit_size: usize::MAX,
        limit_class: None,
    };
    let empty_lits_max = literals_max.to_empty();
    assert_eq!(empty_lits_max.limit_size, usize::MAX);
    assert_eq!(empty_lits_max.limit_class, None);
}

