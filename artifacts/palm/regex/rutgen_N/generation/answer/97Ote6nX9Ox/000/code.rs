// Answer 0

#[derive(Debug)]
struct Sparse {
    size: usize,
}

impl Sparse {
    pub fn new(size: usize) -> Self {
        Sparse { size }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[test]
fn test_len_zero() {
    let sparse = Sparse::new(0);
    assert_eq!(sparse.len(), 0);
}

#[test]
fn test_len_non_zero() {
    let sparse = Sparse::new(5);
    assert_eq!(sparse.len(), 5);
}

#[test]
fn test_len_large_number() {
    let sparse = Sparse::new(1000000);
    assert_eq!(sparse.len(), 1000000);
}

