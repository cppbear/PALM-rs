// Answer 0

#[test]
fn test_next_u32() {
    struct MockRng {
        value: u32,
    }

    impl MockRng {
        fn new(value: u32) -> Self {
            MockRng { value }
        }
    }

    impl rand_core::RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            0 // Implementation not relevant for this test
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {}

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::error::Error> {
            Ok(())
        }
    }

    let mut rng = MockRng::new(42);
    let result = rng.next_u32();
    assert_eq!(result, 42);

    let mut rng_zero = MockRng::new(0);
    let result_zero = rng_zero.next_u32();
    assert_eq!(result_zero, 0);
}

