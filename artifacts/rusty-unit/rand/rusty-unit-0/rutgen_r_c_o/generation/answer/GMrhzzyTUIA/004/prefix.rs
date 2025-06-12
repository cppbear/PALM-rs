// Answer 0

#[test]
fn test_fill_bytes_via_next_length_8() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Not used in this test
        }

        fn drop(&mut self) {}
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut output = [0u8; 8];
    fill_bytes_via_next(&mut rng, &mut output);
}

#[test]
fn test_fill_bytes_via_next_length_9() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Not used in this test
        }

        fn drop(&mut self) {}
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut output = [0u8; 9];
    fill_bytes_via_next(&mut rng, &mut output);
}

#[test]
fn test_fill_bytes_via_next_length_10() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Not used in this test
        }

        fn drop(&mut self) {}
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut output = [0u8; 10];
    fill_bytes_via_next(&mut rng, &mut output);
}

#[test]
fn test_fill_bytes_via_next_length_11() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Not used in this test
        }

        fn drop(&mut self) {}
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut output = [0u8; 11];
    fill_bytes_via_next(&mut rng, &mut output);
}

#[test]
fn test_fill_bytes_via_next_length_12() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Not used in this test
        }

        fn drop(&mut self) {}
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut output = [0u8; 12];
    fill_bytes_via_next(&mut rng, &mut output);
}

