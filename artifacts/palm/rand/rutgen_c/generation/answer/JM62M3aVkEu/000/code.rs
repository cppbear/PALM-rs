// Answer 0

#[test]
fn test_next_u64_via_fill() {
    struct MockRng {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockRng {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.next_u64() as u32;
            value
        }
        
        fn next_u64(&mut self) -> u64 {
            let value = self.bytes[self.position..self.position + 8]
                .iter()
                .enumerate()
                .fold(0u64, |acc, (i, &b)| acc | (b as u64) << (i * 8));
            self.position += 8;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let end_pos = self.position + dest.len();
            dest.copy_from_slice(&self.bytes[self.position..end_pos]);
            self.position += dest.len();
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), core::num::NonZeroUsize> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng::new(vec![1, 0, 0, 0, 0, 0, 0, 0]);
    let result = next_u64_via_fill(&mut rng);
    assert_eq!(result, 1);
}

#[test]
fn test_next_u64_via_fill_empty_bytes() {
    struct MockRng {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockRng {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.next_u64() as u32;
            value
        }

        fn next_u64(&mut self) -> u64 {
            let value = self.bytes[self.position..self.position + 8]
                .iter()
                .enumerate()
                .fold(0u64, |acc, (i, &b)| acc | (b as u64) << (i * 8));
            self.position += 8;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let end_pos = self.position + dest.len();
            dest.copy_from_slice(&self.bytes[self.position..end_pos]);
            self.position += dest.len();
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), core::num::NonZeroUsize> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let bytes: Vec<u8> = vec![];
    let mut rng = MockRng::new(bytes);
    let result = next_u64_via_fill(&mut rng);
    assert_eq!(result, 0);
}

