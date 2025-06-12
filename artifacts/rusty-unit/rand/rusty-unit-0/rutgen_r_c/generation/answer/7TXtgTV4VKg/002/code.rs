// Answer 0

#[test]
fn test_next_u64_index_equals_len_minus_one() {
    struct TestBlockRngCore {
        results: Vec<u32>,
    }
    
    impl Default for TestBlockRngCore {
        fn default() -> Self {
            TestBlockRngCore { results: vec![1, 2, 3, 4] } // len is 4
        }
    }
    
    impl BlockRngCore for TestBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let mut core = TestBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);
    
    block_rng.index = 3; // len - 1
    let result = block_rng.next_u64();
    assert_eq!(result, ((2u64 << 32) | 1u64)); // Expecting (2 << 32) | 1
}

#[test]
fn test_next_u64_index_equals_len() {
    struct TestBlockRngCore {
        results: Vec<u32>,
    }
    
    impl Default for TestBlockRngCore {
        fn default() -> Self {
            TestBlockRngCore { results: vec![1, 2, 3, 4] } // len is 4
        }
    }
    
    impl BlockRngCore for TestBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let mut core = TestBlockRngCore::default();
    let mut block_rng = BlockRng::new(core);

    block_rng.index = 4; // len
    let result = block_rng.next_u64();
    assert_eq!(result, ((4u64 << 32) | 3u64)); // Expecting (3 << 32) | 4
}

