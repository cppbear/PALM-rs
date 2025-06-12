// Answer 0

#[test]
fn test_next_u64() {
    struct TestBlockRng;

    impl BlockRngCore for TestBlockRng {
        type Item = u64;
        type Results = [u64; 4];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&[1, 2, 3, 4]);
        }
    }

    let mut rng = BlockRng64 {
        results: Default::default(),
        index: 0,
        half_used: false,
        core: TestBlockRng,
    };

    assert_eq!(rng.next_u64(), 1);
    assert_eq!(rng.next_u64(), 2);
    assert_eq!(rng.next_u64(), 3);
    assert_eq!(rng.next_u64(), 4);
    
    // Check that generating again refills results
    assert_eq!(rng.next_u64(), 1);
}

#[test]
fn test_next_u64_wrap_around() {
    struct TestBlockRng;

    impl BlockRngCore for TestBlockRng {
        type Item = u64;
        type Results = [u64; 4];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&[10, 20, 30, 40]);
        }
    }

    let mut rng = BlockRng64 {
        results: Default::default(),
        index: 0,
        half_used: false,
        core: TestBlockRng,
    };

    for _ in 0..4 {
        rng.next_u64();
    }
    
    // At this point, the index should have wrapped around
    // and the next call should refill the results
    assert_eq!(rng.next_u64(), 10);
}

