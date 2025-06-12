// Answer 0

#[derive(Debug)]
struct Sparse {
    dense: Vec<i32>,
    size: usize,
}

impl Sparse {
    fn new(dense: Vec<i32>, size: usize) -> Self {
        Sparse { dense, size }
    }

    fn deref(&self) -> &[i32] {
        &self.dense[0..self.size]
    }
}

#[test]
fn test_deref_non_empty() {
    let sparse = Sparse::new(vec![1, 2, 3, 4, 5], 3);
    let result = sparse.deref();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_deref_full() {
    let sparse = Sparse::new(vec![10, 20, 30], 3);
    let result = sparse.deref();
    assert_eq!(result, &[10, 20, 30]);
}

#[test]
fn test_deref_zero_size() {
    let sparse = Sparse::new(vec![100, 200, 300], 0);
    let result = sparse.deref();
    assert_eq!(result, &[]);
}

