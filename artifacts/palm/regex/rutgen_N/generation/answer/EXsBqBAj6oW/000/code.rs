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

    let match_instance = Match { start: 5 };
    assert_eq!(match_instance.start(), 5);
}

#[test]
fn test_start_function_zero() {
    struct Match {
        start: usize,
    }

    impl Match {
        pub fn start(&self) -> usize {
            self.start
        }
    }

    let match_instance = Match { start: 0 };
    assert_eq!(match_instance.start(), 0);
}

#[test]
fn test_start_function_large_value() {
    struct Match {
        start: usize,
    }

    impl Match {
        pub fn start(&self) -> usize {
            self.start
        }
    }

    let match_instance = Match { start: usize::MAX };
    assert_eq!(match_instance.start(), usize::MAX);
}

