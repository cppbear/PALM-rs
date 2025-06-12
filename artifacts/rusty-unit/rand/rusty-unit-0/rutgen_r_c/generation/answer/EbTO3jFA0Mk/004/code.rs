// Answer 0

#[test]
fn test_sample_array_equal_length() {
    struct TestRng {
        counter: usize,
    }

    impl TestRng {
        fn new() -> Self {
            TestRng { counter: 0 }
        }
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.counter += 1;
            range.start // Always return the start for consistent testing
        }
    }

    let mut rng = TestRng::new();
    let len = 5;
    let n = 5; // N == len
    let result = sample_array::<TestRng, 5>(&mut rng, len);
    assert!(result.is_some());
    let indices = result.unwrap();
    assert_eq!(indices.len(), 5);
}

#[test]
fn test_sample_array_larger_n() {
    struct TestRng {
        counter: usize,
    }

    impl TestRng {
        fn new() -> Self {
            TestRng { counter: 0 }
        }
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.counter += 1;
            range.start // Always return the start for consistent testing
        }
    }

    let mut rng = TestRng::new();
    let len = 5;
    let n = 6; // N > len
    let result = sample_array::<TestRng, 6>(&mut rng, len);
    assert!(result.is_none());
}

