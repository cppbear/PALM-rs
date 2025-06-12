// Answer 0

#[test]
fn test_contains_valid_value() {
    struct SparseVec {
        sparse: Vec<usize>,
        dense: Vec<usize>,
        size: usize,
    }

    impl SparseVec {
        pub fn contains(&self, value: usize) -> bool {
            let i = self.sparse[value]; // potential panic here
            i < self.size && self.dense[i] == value
        }
    }

    let sparse_vec = SparseVec {
        sparse: vec![0, 1, 2, 3],
        dense: vec![3, 0, 1, 2],
        size: 4,
    };

    // Test with a valid value that exists in the sparse and dense arrays
    assert!(sparse_vec.contains(3));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_contains_invalid_value_too_high() {
    struct SparseVec {
        sparse: Vec<usize>,
        dense: Vec<usize>,
        size: usize,
    }

    impl SparseVec {
        pub fn contains(&self, value: usize) -> bool {
            let i = self.sparse[value]; // potential panic here
            i < self.size && self.dense[i] == value
        }
    }

    let sparse_vec = SparseVec {
        sparse: vec![0, 1, 2],
        dense: vec![2, 0, 1],
        size: 3,
    };

    // Test with an out-of-bounds value, which should panic
    sparse_vec.contains(5);
}

#[test]
fn test_contains_boundary_case() {
    struct SparseVec {
        sparse: Vec<usize>,
        dense: Vec<usize>,
        size: usize,
    }

    impl SparseVec {
        pub fn contains(&self, value: usize) -> bool {
            let i = self.sparse[value]; // potential panic here
            i < self.size && self.dense[i] == value
        }
    }

    let sparse_vec = SparseVec {
        sparse: vec![0, 1, 2, 3],
        dense: vec![3, 0, 1, 2],
        size: 4,
    };

    // Test the boundary case where value is at the maximum index
    assert!(!sparse_vec.contains(2)); // i < self.size && self.dense[i] == value should be false
}

