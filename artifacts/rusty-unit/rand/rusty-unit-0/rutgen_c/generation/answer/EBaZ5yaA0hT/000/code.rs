// Answer 0

#[test]
fn test_next_u32() {
    struct MockRngCore {
        value: u32,
    }

    impl RngCore for MockRngCore {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            0
        }

        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = MockRngCore { value: 42 };
    let result = rng.next_u32();
    assert_eq!(result, 42);
}

#[test]
fn test_next_u32_zero() {
    struct MockRngCore {
        value: u32,
    }

    impl RngCore for MockRngCore {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            0
        }

        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = MockRngCore { value: 0 };
    let result = rng.next_u32();
    assert_eq!(result, 0);
}

