// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRandCore(u8);

    impl TestRandCore {
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), ()> {
            for byte in dst.iter_mut() {
                *byte = self.0; // Fill with the same value for testing
            }
            Ok(())
        }
    }

    let mut rng = TestRandCore(42);
    let mut buffer = [0u8; 10];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer, [42; 10]);
}

#[test]
fn test_fill_bytes_empty() {
    struct TestRandCore(u8);

    impl TestRandCore {
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), ()> {
            for byte in dst.iter_mut() {
                *byte = self.0; // Fill with the same value for testing
            }
            Ok(())
        }
    }

    let mut rng = TestRandCore(99);
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);
    assert!(buffer.is_empty());
}

