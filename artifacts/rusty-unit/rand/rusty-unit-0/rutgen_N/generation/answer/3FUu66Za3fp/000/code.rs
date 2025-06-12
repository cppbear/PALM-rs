// Answer 0

#[derive(Debug)]
struct MockRandCore(u64);

impl MockRandCore {
    fn try_next_u64(&mut self) -> Result<u64, ()> {
        // For the purpose of testing, let's just return the current value and increment it
        let current = self.0;
        self.0 += 1;
        Ok(current)
    }
}

impl MockRandCore {
    fn next_u64(&mut self) -> u64 {
        self.try_next_u64().unwrap()
    }
}

#[test]
fn test_next_u64_initial_value() {
    let mut mock = MockRandCore(0);
    assert_eq!(mock.next_u64(), 0);
}

#[test]
fn test_next_u64_increment() {
    let mut mock = MockRandCore(5);
    assert_eq!(mock.next_u64(), 5);
    assert_eq!(mock.next_u64(), 6);
}

#[test]
fn test_next_u64_large_value() {
    let mut mock = MockRandCore(u64::MAX - 1);
    assert_eq!(mock.next_u64(), u64::MAX - 1);
    assert_eq!(mock.next_u64(), u64::MAX);
}

