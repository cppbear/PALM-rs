// Answer 0

#[test]
fn test_next_u32() {
    struct MockBlockRngCore {
        values: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { values: vec![1, 2, 3, 4, 5] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear();
            results.extend(self.values.iter());
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    assert_eq!(block_rng.next_u32(), 1);
    assert_eq!(block_rng.next_u32(), 2);
    assert_eq!(block_rng.next_u32(), 3);
    assert_eq!(block_rng.next_u32(), 4);
    assert_eq!(block_rng.next_u32(), 5);
}

#[test]
fn test_next_u32_exceed_index() {
    struct MockBlockRngCore {
        values: Vec<u32>,
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { values: vec![1, 2, 3, 4, 5] }
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear();
            results.extend(self.values.iter());
        }
    }

    let core = MockBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    for _ in 0..5 {
        block_rng.next_u32();
    }
    // The next call should wrap around and regenerate the values
    assert_eq!(block_rng.next_u32(), 1);
}

