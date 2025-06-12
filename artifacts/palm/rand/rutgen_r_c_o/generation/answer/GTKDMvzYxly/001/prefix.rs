// Answer 0

#[test]
fn test_fill_bytes_zero_length() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
    }

    let mut rng = TestRng;
    let mut dst: [u8; 0] = [];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_small_length() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 1;
            }
        }
    }

    let mut rng = TestRng;
    let mut dst: [u8; 1] = [0];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_medium_length() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 2;
            }
        }
    }

    let mut rng = TestRng;
    let mut dst: [u8; 10] = [0; 10];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_max_length() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 255;
            }
        }
    }

    let mut rng = TestRng;
    let mut dst: [u8; 1024] = [0; 1024];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_random_values() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for (i, byte) in dst.iter_mut().enumerate() {
                *byte = (i % 256) as u8;
            }
        }
    }

    let mut rng = TestRng;
    let mut dst: [u8; 64] = [0; 64];
    rng.fill_bytes(&mut dst);
}

