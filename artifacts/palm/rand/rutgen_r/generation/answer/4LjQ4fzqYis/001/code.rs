// Answer 0

#[test]
fn test_fill_bytes_small_array() {
    struct TestRandCore(u8);

    impl TestRandCore {
        fn try_fill_bytes(&self, dst: &mut [u8]) -> Result<(), &'static str> {
            if dst.len() == 0 { return Err("empty buffer"); }
            for byte in dst.iter_mut() {
                *byte = self.0; // Fill with a constant value for testing
            }
            Ok(())
        }
    }

    let mut buffer = [0u8; 5];
    let mut rand_core = TestRandCore(42);
    rand_core.fill_bytes(&mut buffer);
    assert_eq!(buffer, [42, 42, 42, 42, 42]);
}

#[test]
#[should_panic(expected = "empty buffer")]
fn test_fill_bytes_empty_array() {
    struct TestRandCore(u8);

    impl TestRandCore {
        fn try_fill_bytes(&self, dst: &mut [u8]) -> Result<(), &'static str> {
            if dst.len() == 0 { return Err("empty buffer"); }
            for byte in dst.iter_mut() {
                *byte = self.0;
            }
            Ok(())
        }
    }

    let mut buffer: [u8; 0] = [];
    let mut rand_core = TestRandCore(42);
    rand_core.fill_bytes(&mut buffer);
}

