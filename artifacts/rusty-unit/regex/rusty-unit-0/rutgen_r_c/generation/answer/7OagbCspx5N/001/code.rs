// Answer 0

#[test]
fn test_new_single_byte_set() {
    let single_byte_set = SingleByteSet::new();
    
    assert_eq!(single_byte_set.sparse.len(), 256);
    for &value in &single_byte_set.sparse {
        assert_eq!(value, false);
    }
    assert_eq!(single_byte_set.dense.len(), 0);
    assert!(single_byte_set.complete);
    assert!(single_byte_set.all_ascii);
}

