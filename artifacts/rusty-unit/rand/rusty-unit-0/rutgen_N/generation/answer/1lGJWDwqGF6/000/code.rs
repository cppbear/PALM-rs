// Answer 0

#[test]
fn test_next_u64_via_u32() {
    struct MockRng {
        values: Vec<u32>,
        index: usize,
    }

    impl MockRng {
        fn new(values: Vec<u32>) -> Self {
            Self { values, index: 0 }
        }
    }

    impl rand_core::RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.values[self.index];
            self.index += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            unimplemented!() // Not required for this test
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            unimplemented!() // Not required for this test
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            unimplemented!() // Not required for this test
        }
    }

    let mut rng = MockRng::new(vec![0x01234567, 0x89ABCDEF]);
    let result = next_u64_via_u32(&mut rng);
    assert_eq!(result, 0x89ABCDEF01234567);
}

#[test]
fn test_next_u64_via_u32_with_different_values() {
    struct MockRng {
        values: Vec<u32>,
        index: usize,
    }

    impl MockRng {
        fn new(values: Vec<u32>) -> Self {
            Self { values, index: 0 }
        }
    }

    impl rand_core::RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.values[self.index];
            self.index += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            unimplemented!() // Not required for this test
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            unimplemented!() // Not required for this test
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            unimplemented!() // Not required for this test
        }
    }

    let mut rng = MockRng::new(vec![0xFFFFFFFF, 0x00000000]);
    let result = next_u64_via_u32(&mut rng);
    assert_eq!(result, 0x00000000FFFFFFFF);
}

