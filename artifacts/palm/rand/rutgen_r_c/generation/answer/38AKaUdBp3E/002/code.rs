// Answer 0

#[test]
fn test_fill_bytes_basic() {
    struct MockBlockRngCore {
        value: Vec<u64>,
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.value.iter().cloned());
        }
    }

    let mut rng_core = MockBlockRngCore { value: vec![1, 2, 3, 4, 5] };
    let mut block_rng = BlockRng64 {
        results: vec![0; 10],
        index: 0,
        half_used: false,
        core: rng_core,
    };

    let mut buffer = [0u8; 32];
    block_rng.fill_bytes(&mut buffer);

    assert_eq!(&buffer[0..8], &1u64.to_le_bytes());
    assert_eq!(&buffer[8..16], &2u64.to_le_bytes());
    assert_eq!(&buffer[16..24], &3u64.to_le_bytes());
    assert_eq!(&buffer[24..32], &4u64.to_le_bytes());
}

#[test]
fn test_fill_bytes_panic_conditions() {
    struct MockBlockRngCorePanic {
        value: Vec<u64>,
    }

    impl BlockRngCore for MockBlockRngCorePanic {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.value.iter().cloned());
        }
    }

    let mut rng_core = MockBlockRngCorePanic { value: vec![0] };
    let mut block_rng = BlockRng64 {
        results: vec![0; 1],
        index: 0,
        half_used: false,
        core: rng_core,
    };

    let mut buffer = [0u8; 8];
    block_rng.fill_bytes(&mut buffer);

    // Here we ensure we still don't panic while consuming from results
    // Expect that fill_bytes only works properly with the right constraints.
    assert_eq!(&buffer[..], &[0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
#[should_panic]
fn test_fill_bytes_out_of_bounds() {
    struct PanicBlockRngCore;

    impl BlockRngCore for PanicBlockRngCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.push(1);
        }
    }

    let mut rng_core = PanicBlockRngCore {};
    let mut block_rng = BlockRng64 {
        results: vec![0; 1],
        index: 0,
        half_used: false,
        core: rng_core,
    };

    let mut buffer = [0u8; 8];
    block_rng.fill_bytes(&mut buffer);
}

