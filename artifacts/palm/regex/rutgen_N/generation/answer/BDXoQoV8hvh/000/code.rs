// Answer 0

#[derive(Debug)]
struct Matcher {
    end: usize,
}

impl Matcher {
    pub fn new(end: usize) -> Self {
        Matcher { end }
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[test]
fn test_end_with_zero() {
    let matcher = Matcher::new(0);
    assert_eq!(matcher.end(), 0);
}

#[test]
fn test_end_with_positive_value() {
    let matcher = Matcher::new(10);
    assert_eq!(matcher.end(), 10);
}

#[test]
fn test_end_with_large_value() {
    let matcher = Matcher::new(usize::MAX);
    assert_eq!(matcher.end(), usize::MAX);
}

