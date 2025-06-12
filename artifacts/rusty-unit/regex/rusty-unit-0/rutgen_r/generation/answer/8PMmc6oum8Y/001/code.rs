// Answer 0

#[test]
fn test_insert_within_capacity() {
    struct Sparse {
        dense: Vec<usize>,
        sparse: Vec<isize>,
        size: usize,
    }

    impl Sparse {
        fn new(capacity: usize) -> Self {
            Sparse {
                dense: vec![0; capacity],
                sparse: vec![-1; capacity], // using -1 to indicate empty
                size: 0,
            }
        }

        pub fn insert(&mut self, value: usize) {
            let i = self.size;
            self.dense[i] = value;
            self.sparse[value] = i as isize;
            self.size += 1;
        }
    }

    let mut sparse = Sparse::new(10);
    sparse.insert(0);
    sparse.insert(1);
    sparse.insert(2);
    
    assert_eq!(sparse.dense[0], 0);
    assert_eq!(sparse.dense[1], 1);
    assert_eq!(sparse.dense[2], 2);
    assert_eq!(sparse.sparse[0], 0);
    assert_eq!(sparse.sparse[1], 1);
    assert_eq!(sparse.sparse[2], 2);
}

#[should_panic]
#[test]
fn test_insert_panic_dense_overflow() {
    struct Sparse {
        dense: Vec<usize>,
        sparse: Vec<isize>,
        size: usize,
    }

    impl Sparse {
        fn new(capacity: usize) -> Self {
            Sparse {
                dense: vec![0; capacity],
                sparse: vec![-1; capacity], // using -1 to indicate empty
                size: 0,
            }
        }

        pub fn insert(&mut self, value: usize) {
            let i = self.size;
            self.dense[i] = value;
            self.sparse[value] = i as isize;
            self.size += 1;
        }
    }

    let mut sparse = Sparse::new(2); // capacity is 2
    sparse.insert(0);
    sparse.insert(1);
    sparse.insert(2); // This will panic
}

#[should_panic]
#[test]
fn test_insert_panic_sparse_out_of_bounds() {
    struct Sparse {
        dense: Vec<usize>,
        sparse: Vec<isize>,
        size: usize,
    }

    impl Sparse {
        fn new(capacity: usize) -> Self {
            Sparse {
                dense: vec![0; capacity],
                sparse: vec![-1; capacity],
                size: 0,
            }
        }

        pub fn insert(&mut self, value: usize) {
            let i = self.size;
            self.dense[i] = value;
            self.sparse[value] = i as isize;
            self.size += 1;
        }
    }

    let mut sparse = Sparse::new(10);
    sparse.insert(10); // This value will cause panic due to out of bounds on sparse
}

