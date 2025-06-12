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
fn test_next_u32_initial_value() {
    let mut rng = MockRng::default();
    assert_eq!(rng.next_u32(), 1);
}

#[test]
fn test_next_u32_sequential_values() {
    let mut rng = MockRng::default();
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 2);
    assert_eq!(rng.next_u32(), 3);
}

#[test]
fn test_next_u32_large_value() {
    let mut rng = MockRng(100_000);
    assert_eq!(rng.next_u32(), 100_001);
    assert_eq!(rng.next_u32(), 100_002);
}

