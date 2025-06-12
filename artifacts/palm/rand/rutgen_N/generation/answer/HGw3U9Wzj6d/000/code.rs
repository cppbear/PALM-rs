// Answer 0

#[test]
fn test_next_u64() {
    struct MockRng {
        value: u64,
    }

    impl MockRng {
        fn new(value: u64) -> Self {
            MockRng { value }
        }
    }

    impl rand_core::RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            (self.value & 0xFFFFFFFF) as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.value % 256) as u8;
                self.value >>= 8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng::new(42);
    let result = rng.next_u64();
    assert_eq!(result, 42);
}

#[test]
fn test_next_u64_zero() {
    struct MockRng {
        value: u64,
    }

    impl MockRng {
        fn new(value: u64) -> Self {
            MockRng { value }
        }
    }

    impl rand_core::RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            (self.value & 0xFFFFFFFF) as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.value % 256) as u8;
                self.value >>= 8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng::new(0);
    let result = rng.next_u64();
    assert_eq!(result, 0);
}

