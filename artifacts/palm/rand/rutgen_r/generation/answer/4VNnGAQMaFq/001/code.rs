// Answer 0

#[derive(Default)]
struct MockRng(u32);

impl MockRng {
    fn next_u32(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

#[test]
fn test_next_u32_increments_correctly() {
    let mut rng = MockRng::default();
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 2);
    assert_eq!(rng.next_u32(), 3);
}

#[test]
fn test_next_u32_max_value() {
    let mut rng = MockRng(u32::MAX - 1);
    assert_eq!(rng.next_u32(), u32::MAX); // Test overflow case
}

#[should_panic]
#[test]
fn test_next_u32_overflow() {
    let mut rng = MockRng(u32::MAX);
    let _ = rng.next_u32(); // This should panic due to exceeding u32
}

