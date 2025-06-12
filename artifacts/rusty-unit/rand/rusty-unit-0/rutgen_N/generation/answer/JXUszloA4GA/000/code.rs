// Answer 0

#[derive(Debug)]
struct MockRng {
    v: u64,
    a: u64,
}

impl MockRng {
    fn new(v: u64, a: u64) -> Self {
        MockRng { v, a }
    }

    fn next_u64(&mut self) -> u64 {
        let res = self.v;
        self.v = self.v.wrapping_add(self.a);
        res
    }
}

#[test]
fn test_next_u64_initial_value() {
    let mut rng = MockRng::new(5, 3);
    assert_eq!(rng.next_u64(), 5);
}

#[test]
fn test_next_u64_after_increment() {
    let mut rng = MockRng::new(5, 3);
    rng.next_u64(); // should return 5
    assert_eq!(rng.next_u64(), 8); // next should return 8
}

#[test]
fn test_next_u64_wrapping() {
    let mut rng = MockRng::new(u64::MAX, 1);
    assert_eq!(rng.next_u64(), u64::MAX);
    assert_eq!(rng.next_u64(), 0); // should wrap around
}

#[test]
fn test_next_u64_with_negative_increment() {
    let mut rng = MockRng::new(10, 0);
    assert_eq!(rng.next_u64(), 10); // increment of 0 should keep it the same
}

