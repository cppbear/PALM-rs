// Answer 0

#[derive(Default)]
struct Sparse {
    size: usize,
}

impl Sparse {
    pub fn clear(&mut self) {
        self.size = 0;
    }
}

#[test]
fn test_clear_empty_sparse() {
    let mut sparse = Sparse::default();
    sparse.clear();
    assert_eq!(sparse.size, 0);
}

#[test]
fn test_clear_non_empty_sparse() {
    let mut sparse = Sparse { size: 5 };
    sparse.clear();
    assert_eq!(sparse.size, 0);
}

#[test]
fn test_clear_large_sparse() {
    let mut sparse = Sparse { size: 1000 };
    sparse.clear();
    assert_eq!(sparse.size, 0);
}

#[test]
fn test_clear_boundary_case() {
    let mut sparse = Sparse { size: usize::MAX };
    sparse.clear();
    assert_eq!(sparse.size, 0);
}

