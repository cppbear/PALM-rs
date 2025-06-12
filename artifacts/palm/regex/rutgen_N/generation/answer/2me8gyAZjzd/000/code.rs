// Answer 0

#[derive(Debug)]
struct TestSet {
    limit_size: usize,
}

impl TestSet {
    pub fn new(limit_size: usize) -> Self {
        TestSet { limit_size }
    }

    pub fn limit_size(&self) -> usize {
        self.limit_size
    }
}

#[test]
fn test_limit_size_zero() {
    let test_set = TestSet::new(0);
    assert_eq!(test_set.limit_size(), 0);
}

#[test]
fn test_limit_size_positive() {
    let test_set = TestSet::new(100);
    assert_eq!(test_set.limit_size(), 100);
}

#[test]
fn test_limit_size_large() {
    let test_set = TestSet::new(1_000_000);
    assert_eq!(test_set.limit_size(), 1_000_000);
}

