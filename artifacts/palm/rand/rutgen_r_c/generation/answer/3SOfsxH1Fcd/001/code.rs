// Answer 0

#[cfg(test)]
mod tests {
    use crate::RngCore;

    struct MockRng {
        bytes: Vec<u8>,
        index: usize,
    }

    impl MockRng {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, index: 0 }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0 // Not used in this test
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let len = dest.len();
            dest.copy_from_slice(&self.bytes[self.index..self.index + len]);
            self.index += len;
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    #[test]
    fn test_next_u32_via_fill_basic() {
        let mut rng = MockRng::new(vec![1, 2, 3, 4]);
        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, 0x04030201);
    }

    #[test]
    fn test_next_u32_via_fill_boundary_case() {
        let mut rng = MockRng::new(vec![0; 4]); // All zeros
        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_next_u32_via_fill_max_value() {
        let mut rng = MockRng::new(vec![255, 255, 255, 255]); // Max u32 bytes
        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, u32::MAX);
    }

    #[should_panic]
    fn test_next_u32_via_fill_overflow() {
        let mut rng = MockRng::new(vec![1, 2]); // Less than 4 bytes
        let _result = next_u32_via_fill(&mut rng);
    }
}

