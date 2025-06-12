// Answer 0

#[test]
fn test_start_function() {
    struct Match {
        start: usize,
    }

    impl Match {
        pub fn start(&self) -> usize {
            self.start
        }
    }

    // Test with a regular starting point
    let match_1 = Match { start: 5 };
    assert_eq!(match_1.start(), 5);

    // Test with a zero starting point
    let match_2 = Match { start: 0 };
    assert_eq!(match_2.start(), 0);

    // Test with a large starting point
    let match_3 = Match { start: usize::MAX };
    assert_eq!(match_3.start(), usize::MAX);
}

