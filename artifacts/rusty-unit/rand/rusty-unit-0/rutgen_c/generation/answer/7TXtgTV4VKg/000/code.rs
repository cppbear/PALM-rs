// Answer 0

#[test]
fn test_next_u64_index_within_bounds() {
    struct DummyBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for DummyBlockRngCore {
        fn default() -> Self {
            DummyBlockRngCore {
                results: vec![1, 2, 3, 4],
            }
        }
    }

    impl BlockRngCore for DummyBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = DummyBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 0;
    let result = block_rng.next_u64();
    assert_eq!(result, 0x0000000200000001); // 0x00000002 << 32 | 0x00000001
}

#[test]
fn test_next_u64_index_at_end() {
    struct DummyBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for DummyBlockRngCore {
        fn default() -> Self {
            DummyBlockRngCore {
                results: vec![1, 2, 3, 4],
            }
        }
    }

    impl BlockRngCore for DummyBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = DummyBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 2; // Set index to end
    let result = block_rng.next_u64();
    assert_eq!(result, 0x0000000400000003); // 0x00000004 << 32 | 0x00000003
}

#[test]
fn test_next_u64_index_beyond_length() {
    struct DummyBlockRngCore {
        results: Vec<u32>,
    }

    impl Default for DummyBlockRngCore {
        fn default() -> Self {
            DummyBlockRngCore {
                results: vec![1, 2, 3, 4],
            }
        }
    }

    impl BlockRngCore for DummyBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = DummyBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 5; // Set index beyond length
    let result = block_rng.next_u64();
    assert_eq!(result, 0x0000000400000001); // Should regenerate
}

