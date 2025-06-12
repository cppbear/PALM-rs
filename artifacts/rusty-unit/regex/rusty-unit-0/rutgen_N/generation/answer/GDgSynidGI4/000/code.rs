// Answer 0

#[derive(Debug)]
struct Match {
    end: usize,
}

impl Match {
    pub fn new(end: usize) -> Self {
        Match { end }
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[test]
fn test_end() {
    let m = Match::new(5);
    assert_eq!(m.end(), 5);
}

#[test]
fn test_end_zero() {
    let m = Match::new(0);
    assert_eq!(m.end(), 0);
}

#[test]
fn test_end_large_value() {
    let m = Match::new(usize::MAX);
    assert_eq!(m.end(), usize::MAX);
}

