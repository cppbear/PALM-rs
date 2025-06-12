// Answer 0

#[test]
fn test_sample_array_when_n_equals_len_and_non_zero() {
    struct MockRng {
        calls: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::RangeInclusive<usize>) -> usize {
            self.calls += 1;
            *range.start() // For simplicity, always return the start of the range
        }
    }

    let len = 5;
    let n = 5;
    let mut rng = MockRng { calls: 0 };
    let result = sample_array::<MockRng, 5>(&mut rng, len);
}

#[test]
fn test_sample_array_when_n_equals_len_and_zero() {
    struct MockRng {
        calls: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::RangeInclusive<usize>) -> usize {
            self.calls += 1;
            *range.start() // For simplicity, always return the start of the range
        }
    }

    let len = 0;
    let n = 0;
    let mut rng = MockRng { calls: 0 };
    let result = sample_array::<MockRng, 0>(&mut rng, len);
}

#[test]
fn test_sample_array_with_edge_case_max_len() {
    struct MockRng {
        calls: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::RangeInclusive<usize>) -> usize {
            self.calls += 1;
            *range.start() // For simplicity, always return the start of the range
        }
    }

    let len = 100;
    let n = 100;
    let mut rng = MockRng { calls: 0 };
    let result = sample_array::<MockRng, 100>(&mut rng, len);
}

#[test]
fn test_sample_array_with_multiple_calls() {
    struct MockRng {
        calls: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::RangeInclusive<usize>) -> usize {
            self.calls += 1;
            *range.start() // For simplicity, always return the start of the range
        }
    }

    let len = 10;
    let n = 10;
    let mut rng = MockRng { calls: 0 };
    let result_1 = sample_array::<MockRng, 10>(&mut rng, len);
    let result_2 = sample_array::<MockRng, 10>(&mut rng, len);
}

