// Answer 0

#[derive(Debug)]
struct Match {
    start: usize,
}

impl Match {
    pub fn new(start: usize) -> Self {
        Match { start }
    }

    pub fn start(&self) -> usize {
        self.start
    }
}

#[test]
fn test_start_with_zero() {
    let m = Match::new(0);
    assert_eq!(m.start(), 0);
}

#[test]
fn test_start_with_small_value() {
    let m = Match::new(5);
    assert_eq!(m.start(), 5);
}

#[test]
fn test_start_with_large_value() {
    let m = Match::new(usize::MAX);
    assert_eq!(m.start(), usize::MAX);
}

#[test]
fn test_start_with_value_one() {
    let m = Match::new(1);
    assert_eq!(m.start(), 1);
}

