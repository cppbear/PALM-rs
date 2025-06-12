// Answer 0

#[test]
fn test_next_u32_via_fill() {
    struct TestRng {
        value: u32,
    }

    impl rand_core::RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let bytes = self.value.to_le_bytes();
            dest.copy_from_slice(&bytes);
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng { value: 0x01020304 };
    let result = next_u32_via_fill(&mut rng);
    assert_eq!(result, 0x01020304);
}

#[test]
fn test_next_u32_via_fill_boundary() {
    struct BoundaryTestRng {
        value: u32,
    }

    impl rand_core::RngCore for BoundaryTestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let bytes = self.value.to_le_bytes();
            dest.copy_from_slice(&bytes);
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = BoundaryTestRng { value: u32::MAX };
    let result = next_u32_via_fill(&mut rng);
    assert_eq!(result, u32::MAX);
}

