// Answer 0

#[test]
fn test_fill_bytes() {
    struct MockRng(u32);
    
    impl MockRng {
        fn fill_bytes(&self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.0 % 256) as u8; // Simple deterministic fill
            }
        }
    }

    let mut rng = MockRng(42);
    let mut buffer = [0u8; 10];
    
    rng.fill_bytes(&mut buffer);
    
    assert_eq!(buffer, [42, 42, 42, 42, 42, 42, 42, 42, 42, 42]);
}

#[test]
fn test_fill_bytes_empty_buffer() {
    struct MockRng(u32);
    
    impl MockRng {
        fn fill_bytes(&self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.0 % 256) as u8; // Simple deterministic fill
            }
        }
    }

    let rng = MockRng(10);
    let mut buffer: [u8; 0] = [];
    
    rng.fill_bytes(&mut buffer);
    
    // Just ensure it does not panic for empty buffer
}

