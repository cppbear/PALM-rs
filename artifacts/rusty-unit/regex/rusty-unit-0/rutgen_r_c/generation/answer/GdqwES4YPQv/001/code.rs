// Answer 0

#[test]
fn test_approximate_size_empty() {
    let single_byte_set = SingleByteSet {
        sparse: Vec::new(),
        dense: Vec::new(),
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.approximate_size(), 0);
}

#[test]
fn test_approximate_size_with_dense() {
    let single_byte_set = SingleByteSet {
        sparse: Vec::new(),
        dense: vec![1, 2, 3, 4, 5],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.approximate_size(), 5 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_with_sparse() {
    let single_byte_set = SingleByteSet {
        sparse: vec![true, false, true],
        dense: Vec::new(),
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.approximate_size(), 3 * std::mem::size_of::<bool>());
}

#[test]
fn test_approximate_size_with_both_dense_and_sparse() {
    let single_byte_set = SingleByteSet {
        sparse: vec![true, false, true, false],
        dense: vec![1, 2, 3],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.approximate_size(), 
               (3 * std::mem::size_of::<u8>()) + (4 * std::mem::size_of::<bool>()));
}

