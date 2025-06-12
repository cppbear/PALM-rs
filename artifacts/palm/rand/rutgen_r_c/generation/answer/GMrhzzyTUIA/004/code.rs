// Answer 0

#[test]
fn test_fill_bytes_via_next_large_slice() {
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

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), std::io::Error> {
            unimplemented!()
        }
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut dest = [0u8; 16]; // Length > 8, will require multiple next_u64 calls.

    fill_bytes_via_next(&mut rng, &mut dest);

    assert_eq!(dest, [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 0xef, 0xcd, 0xab, 0x89]);
}

#[test]
fn test_fill_bytes_via_next_edge_case() {
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

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), std::io::Error> {
            unimplemented!()
        }
    }

    let mut rng = TestRng { value: 0x0123456789abcdef };
    let mut dest = [0u8; 5]; // Length <= 4, should fill using next_u32.

    fill_bytes_via_next(&mut rng, &mut dest);

    assert_eq!(dest, [0xef, 0xcd, 0xab, 0x89, 0x67]); // Last 5 bytes filled.
}

#[test]
fn test_fill_bytes_via_next_zero_length() {
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

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), std::io::Error> {
            unimplemented!()
        }
    }

    let mut rng = TestRng { value: 0 };
    let mut dest: &mut [u8] = &mut []; // Zero-length slice.

    fill_bytes_via_next(&mut rng, &mut dest); // Should not panic.
}

