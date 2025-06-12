// Answer 0

#[test]
fn test_next_u32_with_valid_index() {
    struct TestCore {
        values: [u32; 5],
        current: usize,
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = [u32; 5];
        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.values);
        }
    }

    let core = TestCore {
        values: [1, 2, 3, 4, 5],
        current: 0,
    };

    let mut block_rng = BlockRng::new(core);
    block_rng.results = [0; 5]; // Initialize results
    block_rng.index = 0; // Set index to 0 to start

    assert_eq!(block_rng.next_u32(), 1);
    assert_eq!(block_rng.next_u32(), 2);
    assert_eq!(block_rng.next_u32(), 3);
    assert_eq!(block_rng.next_u32(), 4);
    assert_eq!(block_rng.next_u32(), 5); // Should fill and return again
}

#[test]
fn test_next_u32_triggers_generation() {
    struct TestCore {
        values: [u32; 3],
    }

    impl BlockRngCore for TestCore {
        type Item = u32;
        type Results = [u32; 3];
        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.values);
        }
    }

    let core = TestCore {
        values: [10, 20, 30],
    };

    let mut block_rng = BlockRng::new(core);
    block_rng.results = [0; 3]; // Initialize results
    block_rng.index = 3; // Set index to trigger generation

    assert_eq!(block_rng.next_u32(), 10); // Should generate new values and return 10
    assert_eq!(block_rng.next_u32(), 20);
    assert_eq!(block_rng.next_u32(), 30);
}

