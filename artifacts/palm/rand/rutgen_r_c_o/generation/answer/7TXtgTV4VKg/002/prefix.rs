// Answer 0

#[test]
fn test_next_u64_index_equals_len_minus_one() {
    struct MockBlockRngCore {
        results: [u32; 2],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore { results: [1, 2] };
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 1; // This will trigger index < len - 1 to be false (1 == 1)

    let result = block_rng.next_u64();
}

#[test]
fn test_next_u64_index_equals_len() {
    struct MockBlockRngCore {
        results: [u32; 2],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.results);
        }
    }

    let core = MockBlockRngCore { results: [3, 4] };
    let mut block_rng = BlockRng::new(core);
    block_rng.index = 2; // This will trigger index >= len to be true (2 >= 2)

    let result = block_rng.next_u64();
}

