// Answer 0

#[test]
fn test_fill_bytes_with_exact_size() {
    struct TestBlockRng {
        data: Vec<u32>,
    }

    impl Default for TestBlockRng {
        fn default() -> Self {
            Self {
                data: vec![1, 2, 3, 4],
            }
        }
    }

    impl BlockRngCore for TestBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.data);
        }
    }

    let mut block_rng = BlockRng::new(TestBlockRng::default());
    let mut dest = vec![0u8; 16]; // Size matches 4 u32 values (4 * 4 = 16 bytes)
    block_rng.fill_bytes(&mut dest);

    assert_eq!(dest, vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_partial_fill() {
    struct TestBlockRng {
        data: Vec<u32>,
    }

    impl Default for TestBlockRng {
        fn default() -> Self {
            Self {
                data: vec![1, 2],
            }
        }
    }

    impl BlockRngCore for TestBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&self.data);
        }
    }

    let mut block_rng = BlockRng::new(TestBlockRng::default());
    let mut dest = vec![0u8; 12]; // Size will only fill 3 u32 values
    block_rng.fill_bytes(&mut dest);

    assert_eq!(dest, vec![1, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0]);
}

#[test]
#[should_panic]
fn test_fill_bytes_index_out_of_bounds() {
    struct TestBlockRng {
        data: Vec<u32>,
    }

    impl Default for TestBlockRng {
        fn default() -> Self {
            Self {
                data: vec![],
            }
        }
    }

    impl BlockRngCore for TestBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear(); // Simulate no data available to generate
        }
    }

    let mut block_rng = BlockRng::new(TestBlockRng::default());
    let mut dest = vec![0u8; 8]; // Attempting to fill when data is empty
    block_rng.fill_bytes(&mut dest);
}

