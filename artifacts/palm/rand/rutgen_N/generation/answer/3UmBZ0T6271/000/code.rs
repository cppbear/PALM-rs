// Answer 0

#[derive(Debug)]
struct MockRng(u64);

impl MockRng {
    fn try_next_u64(&mut self) -> Result<u64, ()> {
        // Simulate generating a random u64 and for testing purposes,
        // we'll return the current value of self.0 and then increment it.
        let next_value = self.0;
        self.0 += 1; // Increment for the sake of generating a new value next time
        Ok(next_value)
    }
}

impl MockRng {
    fn next_u64(&mut self) -> u64 {
        self.try_next_u64().unwrap()
    }
}

#[test]
fn test_next_u64_initial_value() {
    let mut rng = MockRng(0);
    assert_eq!(rng.next_u64(), 0);
}

#[test]
fn test_next_u64_increment() {
    let mut rng = MockRng(5);
    assert_eq!(rng.next_u64(), 5);
    assert_eq!(rng.next_u64(), 6);
}

#[test]
fn test_next_u64_large_value() {
    let mut rng = MockRng(u64::MAX - 1);
    assert_eq!(rng.next_u64(), u64::MAX - 1);
    assert_eq!(rng.next_u64(), u64::MAX);
}

