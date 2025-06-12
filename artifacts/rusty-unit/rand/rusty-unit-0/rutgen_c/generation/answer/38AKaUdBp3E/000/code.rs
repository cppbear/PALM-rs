// Answer 0

#[test]
fn test_fill_bytes_full_fill() {
    struct DummyCore {
        count: u64,
    }

    impl BlockRngCore for DummyCore {
        type Item = u64;
        type Results = [u64; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results[0] = self.count;
            results[1] = self.count + 1;
            self.count += 2;
        }
    }

    let mut rng = BlockRng64 {
        results: Default::default(),
        index: 0,
        half_used: false,
        core: DummyCore { count: 0 },
    };

    let mut buffer = [0u8; 16];
    rng.fill_bytes(&mut buffer);

    assert_eq!(&buffer, &[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_partial_fill() {
    struct DummyCore {
        count: u64,
    }

    impl BlockRngCore for DummyCore {
        type Item = u64;
        type Results = [u64; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results[0] = self.count;
            results[1] = self.count + 1;
            self.count += 2;
        }
    }

    let mut rng = BlockRng64 {
        results: Default::default(),
        index: 0,
        half_used: false,
        core: DummyCore { count: 0 },
    };

    let mut buffer = [0u8; 10];
    rng.fill_bytes(&mut buffer);

    assert_eq!(&buffer[..8], &[0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(&buffer[8..], &[1, 0]); // Partially filled with the second u64
}

#[test]
fn test_fill_bytes_empty_dest() {
    struct DummyCore {
        count: u64,
    }

    impl BlockRngCore for DummyCore {
        type Item = u64;
        type Results = [u64; 2];

        fn generate(&mut self, results: &mut Self::Results) {
            results[0] = self.count;
            results[1] = self.count + 1;
            self.count += 2;
        }
    }

    let mut rng = BlockRng64 {
        results: Default::default(),
        index: 0,
        half_used: false,
        core: DummyCore { count: 0 },
    };

    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer); // Should not panic, performing no operations
}

