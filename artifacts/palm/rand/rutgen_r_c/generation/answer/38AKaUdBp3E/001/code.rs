// Answer 0

#[test]
fn test_fill_bytes_full_fill() {
    struct MockBlockRngCore {
        index: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(vec![1u64, 2u64, 3u64]); // Mock values
            self.index += 3;
        }
    }

    let mut results: Vec<u64> = vec![0; 3]; // Enough room for one fill
    let mut rng = BlockRng64 {
        results,
        index: 0,
        half_used: false,
        core: MockBlockRngCore { index: 0 },
    };

    let mut dest = [0u8; 24]; // 3 * 8 bytes for three u64s
    rng.fill_bytes(&mut dest);

    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_partial_fill() {
    struct MockBlockRngCore {
        index: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(vec![9u64]); // Only one item to generate
            self.index += 1;
        }
    }

    let mut results: Vec<u64> = vec![0; 1]; // Space for one u64
    let mut rng = BlockRng64 {
        results,
        index: 0,
        half_used: false,
        core: MockBlockRngCore { index: 0 },
    };

    let mut dest = [0u8; 8]; // Space for one u64
    rng.fill_bytes(&mut dest);

    assert_eq!(&dest, &[9, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_exceeding_capacity() {
    struct MockBlockRngCore {
        index: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(vec![8u64, 7u64]); // Generate two values
            self.index += 2;
        }
    }

    let mut results: Vec<u64> = vec![0; 2]; // Space for two u64s
    let mut rng = BlockRng64 {
        results,
        index: 0,
        half_used: false,
        core: MockBlockRngCore { index: 0 },
    };

    let mut dest = [0u8; 16]; // Space for two u64s
    rng.fill_bytes(&mut dest);

    assert_eq!(&dest, &[8, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
#[should_panic]
fn test_fill_bytes_panics_on_empty_results() {
    struct MockBlockRngCore;

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, _results: &mut Self::Results) {
            panic!("Should not generate");
        }
    }

    let mut results: Vec<u64> = vec![]; // Empty results
    let mut rng = BlockRng64 {
        results,
        index: 0,
        half_used: false,
        core: MockBlockRngCore,
    };

    let mut dest = [0u8; 8]; // Space for one u64
    rng.fill_bytes(&mut dest);
}

#[test]
#[should_panic]
fn test_fill_bytes_panics_on_empty_dest() {
    struct MockBlockRngCore {
        index: usize,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(vec![4u64]); // Generate one value
        }
    }

    let mut results: Vec<u64> = vec![0; 1]; // Space for oneu64
    let mut rng = BlockRng64 {
        results,
        index: 0,
        half_used: false,
        core: MockBlockRngCore { index: 0 },
    };

    let mut dest: Vec<u8> = vec![]; // Empty destination
    rng.fill_bytes(&mut dest);
}

