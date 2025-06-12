// Answer 0

#[test]
fn test_block_rng_reset() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![1, 2, 3, 4, 5],
    };

    let mut block_rng = BlockRng64::new(core);
    block_rng.generate_and_set(0);

    assert_eq!(block_rng.index(), 5);

    block_rng.reset();

    assert_eq!(block_rng.index(), 5);
    assert!(!block_rng.half_used);
}

#[test]
fn test_block_rng_reset_after_half_used() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![10, 20, 30, 40, 50],
    };

    let mut block_rng = BlockRng64::new(core);
    block_rng.generate_and_set(0);

    block_rng.half_used = true;

    assert_eq!(block_rng.index(), 5);
    assert!(block_rng.half_used);

    block_rng.reset();

    assert_eq!(block_rng.index(), 5);
    assert!(!block_rng.half_used);
}

