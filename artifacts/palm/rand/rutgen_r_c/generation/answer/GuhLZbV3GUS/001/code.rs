// Answer 0

#[test]
fn test_next_u32_when_index_is_equal_to_results_length() {
    struct MockBlockRngCore {
        count: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..self.count {
                results.push(i as u32);
            }
        }
    }

    let core = MockBlockRngCore { count: 5 };
    let mut rng = BlockRng {
        results: Vec::with_capacity(5),
        index: 5,
        core,
    };

    let result = rng.next_u32();
    assert_eq!(result, 0); // Expect first value after reset
}

#[test]
fn test_next_u32_when_index_is_within_results_length() {
    struct MockBlockRngCore {
        count: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..self.count {
                results.push(i as u32);
            }
        }
    }

    let core = MockBlockRngCore { count: 5 };
    let mut rng = BlockRng {
        results: vec![0, 1, 2, 3, 4],
        index: 0,
        core,
    };

    let result = rng.next_u32();
    assert_eq!(result, 0); // Expecting the first value in the results
}

#[test]
fn test_next_u32_when_index_equals_results_length_and_generates_new_values() {
    struct MockBlockRngCore {
        count: usize,
        call_count: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear(); // Clear previous values
            for i in 0..self.count {
                results.push((self.call_count * self.count + i) as u32);
            }
            self.call_count += 1;
        }
    }

    let core = MockBlockRngCore { count: 3, call_count: 0 };
    let mut rng = BlockRng {
        results: Vec::new(),
        index: 3,
        core,
    };

    let result = rng.next_u32();
    assert_eq!(result, 0); // Expecting the first new generated value
    rng.next_u32(); // Advance index
    let next_result = rng.next_u32();
    assert_eq!(next_result, 1); // Expecting the next value in the new results
}

