pub fn new(size: usize) -> SparseSet {
        SparseSet {
            dense: vec![0; size],
            sparse: vec![0; size],
            size: 0,
        }
    }