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
fn test_start_function() {
    let m = Match::new(10);
    assert_eq!(m.start(), 10);
}

#[test]
fn test_start_function_zero() {
    let m = Match::new(0);
    assert_eq!(m.start(), 0);
}

#[test]
fn test_start_function_large_value() {
    let m = Match::new(usize::MAX);
    assert_eq!(m.start(), usize::MAX);
}

