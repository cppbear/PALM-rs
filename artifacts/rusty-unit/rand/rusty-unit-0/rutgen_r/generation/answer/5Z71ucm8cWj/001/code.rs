// Answer 0

#[test]
fn test_reset_with_non_empty_results() {
    struct TestRandCore {
        index: usize,
        results: Vec<u8>,
    }

    impl TestRandCore {
        pub fn reset(&mut self) {
            self.index = self.results.len();
        }
    }

    let mut rng = TestRandCore {
        index: 0,
        results: vec![1, 2, 3, 4, 5],
    };

    rng.reset();
    assert_eq!(rng.index, 5);
}

#[test]
fn test_reset_with_empty_results() {
    struct TestRandCore {
        index: usize,
        results: Vec<u8>,
    }

    impl TestRandCore {
        pub fn reset(&mut self) {
            self.index = self.results.len();
        }
    }

    let mut rng = TestRandCore {
        index: 0,
        results: Vec::new(),
    };

    rng.reset();
    assert_eq!(rng.index, 0);
}

