// Answer 0

#[derive(Debug)]
struct MockRandomGenerator(u32);

impl MockRandomGenerator {
    fn try_next_u32(&mut self) -> Result<u32, ()> {
        // For testing purposes, return the current value and then increment it
        let current = self.0;
        self.0 += 1;
        Ok(current)
    }
}

impl MockRandomGenerator {
    fn next_u32(&mut self) -> u32 {
        self.try_next_u32().unwrap()
    }
}

#[test]
fn test_next_u32() {
    let mut rng = MockRandomGenerator(0);
    assert_eq!(rng.next_u32(), 0);
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 2);
}

#[test]
fn test_next_u32_boundary() {
    let mut rng = MockRandomGenerator(u32::MAX);
    assert_eq!(rng.next_u32(), u32::MAX);
    // Should wrap around to 0 after reaching u32::MAX
    assert_eq!(rng.next_u32(), 0);
}

