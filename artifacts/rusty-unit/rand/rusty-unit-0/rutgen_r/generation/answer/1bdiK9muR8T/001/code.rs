// Answer 0

#[derive(Default)]
struct MockRng {
    value: u64,
}

impl MockRng {
    fn next_u64(&mut self) -> u64 {
        self.value
    }
}

#[test]
fn test_next_u32_zero() {
    let mut rng = MockRng::default();
    rng.value = 0;
    assert_eq!(rng.next_u32(), 0);
}

#[test]
fn test_next_u32_max() {
    let mut rng = MockRng::default();
    rng.value = u64::MAX;
    assert_eq!(rng.next_u32(), u32::MAX);
}

#[test]
fn test_next_u32_mid() {
    let mut rng = MockRng::default();
    rng.value = 0x7FFFFFFF;
    assert_eq!(rng.next_u32(), 0x7FFFFFFF);
}

