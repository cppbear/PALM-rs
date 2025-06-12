// Answer 0

struct MockRng(u32);

impl MockRng {
    fn next_u32(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

#[test]
fn test_next_u32_initial_value() {
    let mut rng = MockRng(0);
    assert_eq!(rng.next_u32(), 1); // First call should return 1
}

#[test]
fn test_next_u32_consecutive_calls() {
    let mut rng = MockRng(1);
    assert_eq!(rng.next_u32(), 2); // Second call should return 2
    assert_eq!(rng.next_u32(), 3); // Third call should return 3
}

#[test]
fn test_next_u32_large_initial_value() {
    let mut rng = MockRng(u32::MAX - 1);
    assert_eq!(rng.next_u32(), u32::MAX); // Should wrap to MAX
}

#[test]
fn test_next_u32_maximum_value() {
    let mut rng = MockRng(u32::MAX);
    assert_eq!(rng.next_u32(), u32::MAX + 1); // Testing potential wraparound, where expected is unrealistic
}

#[should_panic(expected = "overflow")]
#[test]
fn test_next_u32_overflow() {
    let mut rng = MockRng(u32::MAX);
    let _ = rng.next_u32(); // This will panic due to overflow
}

