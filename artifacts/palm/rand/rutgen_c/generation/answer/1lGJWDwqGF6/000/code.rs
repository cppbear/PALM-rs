// Answer 0

#[test]
fn test_next_u64_via_u32() {
    struct MockRng {
        values: Vec<u32>,
        index: usize,
    }

    impl MockRng {
        fn new(values: Vec<u32>) -> Self {
            MockRng { values, index: 0 }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.values[self.index];
            self.index += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            u64::from(self.next_u32()) | (u64::from(self.next_u32()) << 32)
        }

        fn take(&mut self, _: usize) {}

        fn fill_bytes(&mut self, _: &mut [u8]) {}

        fn fill_bytes_e(&mut self, _: &mut [u8]) {}
    }

    let mut rng = MockRng::new(vec![0x12345678, 0x9abcdef0]);
    let result = next_u64_via_u32(&mut rng);
    assert_eq!(result, 0x9abcdef012345678);

    let mut rng2 = MockRng::new(vec![0, 1]);
    let result2 = next_u64_via_u32(&mut rng2);
    assert_eq!(result2, 0x0000000100000000);

    let mut rng3 = MockRng::new(vec![0xffffffff, 0xffffffff]);
    let result3 = next_u64_via_u32(&mut rng3);
    assert_eq!(result3, 0xffffffffffffffff);
}

