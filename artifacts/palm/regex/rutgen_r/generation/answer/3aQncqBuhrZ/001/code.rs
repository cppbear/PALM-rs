// Answer 0

#[derive(Debug)]
struct SparseArray {
    sparse: Vec<usize>,
    dense: Vec<usize>,
    size: usize,
}

impl SparseArray {
    pub fn new(sparse: Vec<usize>, dense: Vec<usize>, size: usize) -> Self {
        Self { sparse, dense, size }
    }

    pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        i < self.size && self.dense[i] == value
    }
}

#[test]
fn test_contains_valid_value() {
    let sparse = vec![0, 1, 2];
    let dense = vec![0, 1, 2];
    let size = 3;
    let sparse_array = SparseArray::new(sparse, dense, size);

    assert_eq!(sparse_array.contains(1), true); // should return true
}

#[test]
#[should_panic]
fn test_contains_invalid_index() {
    let sparse = vec![0, 1, 2];
    let dense = vec![0, 1, 2];
    let size = 3;
    let sparse_array = SparseArray::new(sparse, dense, size);

    sparse_array.contains(3); // should panic since '3' is out of bounds
}

#[test]
fn test_contains_boundary_value_low() {
    let sparse = vec![0];
    let dense = vec![0];
    let size = 1;
    let sparse_array = SparseArray::new(sparse, dense, size);

    assert_eq!(sparse_array.contains(0), true); // should return true
}

#[test]
#[should_panic]
fn test_contains_boundary_value_high() {
    let sparse = vec![0];
    let dense = vec![0];
    let size = 1;
    let sparse_array = SparseArray::new(sparse, dense, size);

    sparse_array.contains(1); // should panic since '1' is out of bounds
}

