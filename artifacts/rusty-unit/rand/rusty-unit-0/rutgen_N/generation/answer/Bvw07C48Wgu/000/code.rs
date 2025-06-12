// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRng(u32);

    impl TestRng {
        fn fill_bytes(&self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.0 % 256) as u8; // Simple deterministic behavior
                self.0 = self.0.wrapping_add(1); // Change state
            }
        }
    }

    let mut rng = TestRng(0);
    let mut buffer = [0u8; 10];
    rng.fill_bytes(&mut buffer);

    assert_eq!(buffer, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_fill_bytes_empty_buffer() {
    struct TestRng(u32);

    impl TestRng {
        fn fill_bytes(&self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.0 % 256) as u8;
                self.0 = self.0.wrapping_add(1);
            }
        }
    }

    let mut rng = TestRng(0);
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);

    assert_eq!(buffer, []);
}

