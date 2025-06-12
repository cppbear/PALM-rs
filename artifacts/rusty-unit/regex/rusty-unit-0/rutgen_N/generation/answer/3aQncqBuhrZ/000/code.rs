// Answer 0

#[derive(Debug)]
struct Sparse {
    sparse: Vec<usize>,
    dense: Vec<usize>,
    size: usize,
}

impl Sparse {
    pub fn new(sparse: Vec<usize>, dense: Vec<usize>, size: usize) -> Self {
        Sparse { sparse, dense, size }
    }

    pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        i < self.size && self.dense[i] == value
    }
}

#[test]
fn test_contains_found() {
    let sparse = Sparse::new(vec![0, 1, 2], vec![10, 20, 30], 3);
    assert_eq!(sparse.contains(10), true);
}

#[test]
fn test_contains_not_found() {
    let sparse = Sparse::new(vec![0, 1, 2], vec![10, 20, 30], 3);
    assert_eq!(sparse.contains(40), false);
}

#[test]
fn test_contains_boundary_exceeding_index() {
    let sparse = Sparse::new(vec![0, 1, 2], vec![10, 20, 30], 3);
    assert_eq!(sparse.contains(30), false);
}

#[test]
fn test_contains_boundary_valid_index() {
    let sparse = Sparse::new(vec![0, 1, 2], vec![10, 20, 30], 3);
    assert_eq!(sparse.contains(20), true);
}

