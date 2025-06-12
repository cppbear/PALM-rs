// Answer 0

#[derive(Debug)]
struct MockRng(u32);

impl MockRng {
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(1); // Simulating RNG behavior
        self.0
    }
}

#[test]
fn test_next_u32_basic() {
    let mut rng = MockRng(0);
    let result = rng.next_u32();
    assert_eq!(result, 1);
}

#[test]
fn test_next_u32_boundary() {
    let mut rng = MockRng(u32::MAX);
    let result = rng.next_u32();
    assert_eq!(result, 0); // Testing wrapping behavior
}

#[test]
fn test_next_u32_multiple_calls() {
    let mut rng = MockRng(5);
    assert_eq!(rng.next_u32(), 6);
    assert_eq!(rng.next_u32(), 7);
    assert_eq!(rng.next_u32(), 8);
}

#[test]
fn test_next_u32_large_initial_value() {
    let mut rng = MockRng(1000);
    let result = rng.next_u32();
    assert_eq!(result, 1001);
}

