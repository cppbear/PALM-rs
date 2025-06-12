// Answer 0

#[test]
fn test_approximate_size() {
    struct TestStruct {
        dense: Vec<u8>,
        sparse: Vec<bool>,
    }

    impl TestStruct {
        fn approximate_size(&self) -> usize {
            (self.dense.len() * std::mem::size_of::<u8>())
            + (self.sparse.len() * std::mem::size_of::<bool>())
        }
    }

    let test_instance = TestStruct {
        dense: vec![1, 2, 3, 4, 5],
        sparse: vec![true, false, true],
    };

    let size = test_instance.approximate_size();
    assert_eq!(size, (5 * std::mem::size_of::<u8>()) + (3 * std::mem::size_of::<bool>()));
}

#[test]
fn test_approximate_size_empty() {
    struct TestStruct {
        dense: Vec<u8>,
        sparse: Vec<bool>,
    }

    impl TestStruct {
        fn approximate_size(&self) -> usize {
            (self.dense.len() * std::mem::size_of::<u8>())
            + (self.sparse.len() * std::mem::size_of::<bool>())
        }
    }

    let test_instance = TestStruct {
        dense: vec![],
        sparse: vec![],
    };

    let size = test_instance.approximate_size();
    assert_eq!(size, 0);
}

#[test]
fn test_approximate_size_in_single_element() {
    struct TestStruct {
        dense: Vec<u8>,
        sparse: Vec<bool>,
    }

    impl TestStruct {
        fn approximate_size(&self) -> usize {
            (self.dense.len() * std::mem::size_of::<u8>())
            + (self.sparse.len() * std::mem::size_of::<bool>())
        }
    }

    let test_instance = TestStruct {
        dense: vec![1],
        sparse: vec![true],
    };

    let size = test_instance.approximate_size();
    assert_eq!(size, (1 * std::mem::size_of::<u8>()) + (1 * std::mem::size_of::<bool>()));
}

