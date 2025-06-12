// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42 // Fixed value for testing
        }

        fn next_u64(&mut self) -> u64 {
            42 // Fixed value for testing
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 255; // Fill with a known value for testing
            }
        }
    }

    impl rand_core::SeedableRng for TestRng {
        type Seed = [u8; 32];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let mut rng = TestRng;
    let mut buffer = [0u8; 10];

    rng.fill_bytes(&mut buffer);

    assert_eq!(buffer, [255; 10]);
}

