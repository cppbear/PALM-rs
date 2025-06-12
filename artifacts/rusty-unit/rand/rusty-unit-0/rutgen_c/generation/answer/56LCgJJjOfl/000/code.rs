// Answer 0

#[test]
fn test_fill_bytes() {
    use std::mem::size_of;
    use rand_core::RngCore;

    struct MockCore;
    struct MockOsRng;

    impl RngCore for MockCore {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 1; // Filling with a non-zero value for testing
            }
        }
    }

    impl RngCore for MockOsRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 2; // Different value for another mock
            }
        }
    }

    struct TestReseedingRng<R, Rsdr>(MockCore, MockOsRng)
    where
        R: RngCore,
        Rsdr: RngCore;

    let mut rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(TestReseedingRng(MockCore, MockOsRng))),
    };

    let mut buffer = [0u8; 10];
    rng.fill_bytes(&mut buffer);

    for &byte in &buffer {
        assert_eq!(byte, 1); // Check if the filled bytes are as expected
    }
}

#[test]
fn test_fill_bytes_empty_buffer() {
    let mut rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(MockCore)),
    };
    let mut buffer: Vec<u8> = Vec::new();
    rng.fill_bytes(&mut buffer);
    assert!(buffer.is_empty()); // Ensure no modifications occur with an empty buffer
}

