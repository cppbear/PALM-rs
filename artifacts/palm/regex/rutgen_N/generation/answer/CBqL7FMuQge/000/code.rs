// Answer 0

#[derive(Default)]
struct Sparse {
    size: usize,
}

impl Sparse {
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[test]
fn test_is_empty_when_size_is_zero() {
    let sparse = Sparse::default();
    assert!(sparse.is_empty());
}

#[test]
fn test_is_empty_when_size_is_non_zero() {
    let mut sparse = Sparse::default();
    sparse.size = 1;
    assert!(!sparse.is_empty());
}

