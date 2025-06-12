// Answer 0

#[test]
fn test_next_u32() {
    struct MockRng(u32);

    impl MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.0;
            self.0 += 1;
            value
        }
    }

    let mut rng = MockRng(0);
    assert_eq!(rng.next_u32(), 0);
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 2);
}

#[test]
fn test_next_u32_boundary() {
    struct MockRng(u32);

    impl MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.0;
            self.0 += 1;
            value
        }
    }

    let mut rng = MockRng(u32::MAX - 1);
    assert_eq!(rng.next_u32(), u32::MAX - 1);
    assert_eq!(rng.next_u32(), u32::MAX);
}

