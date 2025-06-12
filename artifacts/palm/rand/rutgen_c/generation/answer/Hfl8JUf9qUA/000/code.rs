// Answer 0

#[test]
fn test_try_fill_bytes() {
    struct TestRng {
        counter: u8,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.counter;
            }
            self.counter = self.counter.wrapping_add(1);
        }
    }

    let mut rng = TestRng { counter: 0 };
    let mut bytes = [0u8; 5];

    let result = rng.try_fill_bytes(&mut bytes);
    assert!(result.is_ok());
    assert_eq!(bytes, [0, 0, 0, 0, 0]);

    let result = rng.try_fill_bytes(&mut bytes);
    assert!(result.is_ok());
    assert_eq!(bytes, [1, 1, 1, 1, 1]);

    let result = rng.try_fill_bytes(&mut bytes);
    assert!(result.is_ok());
    assert_eq!(bytes, [2, 2, 2, 2, 2]);
}

