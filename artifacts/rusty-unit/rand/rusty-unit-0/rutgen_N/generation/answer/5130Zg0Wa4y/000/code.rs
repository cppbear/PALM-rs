// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRand {
        seed: u64,
    }

    impl TestRand {
        fn new(seed: u64) -> Self {
            TestRand { seed }
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRand::new(42);
    let mut buffer = [0u8; 16];

    rng.fill_bytes(&mut buffer);

    assert!(buffer.iter().any(|&b| b != 0), "Buffer should not be all zeros");
}

#[test]
fn test_fill_bytes_empty() {
    struct TestRand {
        seed: u64,
    }

    impl TestRand {
        fn new(seed: u64) -> Self {
            TestRand { seed }
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRand::new(42);
    let mut buffer: [u8; 0] = [];

    rng.fill_bytes(&mut buffer);

    assert!(buffer.is_empty(), "Buffer should remain empty");
}

