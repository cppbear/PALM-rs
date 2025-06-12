// Answer 0

#[derive(Default)]
struct TestRng;

impl rand_core::RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        42
    }

    fn next_u64(&mut self) -> u64 {
        42
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = 1; // Fill with a fixed value for testing
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[test]
fn test_fill_bytes() {
    let mut rng = TestRng::default();
    let mut buffer = [0u8; 10];
    
    rng.fill_bytes(&mut buffer);
    
    assert_eq!(buffer, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_fill_bytes_zero_length() {
    let mut rng = TestRng::default();
    let mut buffer: [u8; 0] = [];
    
    rng.fill_bytes(&mut buffer);
    
    assert_eq!(buffer.len(), 0);
}

