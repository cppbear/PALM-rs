// Answer 0

#[test]
fn test_approximate_size_empty() {
    let byte_set = SingleByteSet {
        sparse: Vec::new(),
        dense: Vec::new(),
        complete: false,
        all_ascii: false,
    };
    assert_eq!(byte_set.approximate_size(), 0);
}

#[test]
fn test_approximate_size_sparse_only() {
    let byte_set = SingleByteSet {
        sparse: vec![true, false, true],
        dense: Vec::new(),
        complete: false,
        all_ascii: false,
    };
    assert_eq!(byte_set.approximate_size(), 3 * std::mem::size_of::<bool>());
}

#[test]
fn test_approximate_size_dense_only() {
    let byte_set = SingleByteSet {
        sparse: Vec::new(),
        dense: vec![1, 2, 3, 4],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(byte_set.approximate_size(), 4 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_dense_and_sparse() {
    let byte_set = SingleByteSet {
        sparse: vec![true, false, true],
        dense: vec![1, 2],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(byte_set.approximate_size(), 
               (2 * std::mem::size_of::<u8>()) + (3 * std::mem::size_of::<bool>()));
}

