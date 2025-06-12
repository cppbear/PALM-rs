// Answer 0

#[test]
fn test_new_single_byte_set() {
    struct SingleByteSet {
        sparse: Vec<bool>,
        dense: Vec<u8>,
        complete: bool,
        all_ascii: bool,
    }

    fn new() -> SingleByteSet {
        SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![],
            complete: true,
            all_ascii: true,
        }
    }

    let single_byte_set = new();
    
    assert_eq!(single_byte_set.sparse.len(), 256);
    assert!(single_byte_set.sparse.iter().all(|&x| !x));
    assert!(single_byte_set.dense.is_empty());
    assert!(single_byte_set.complete);
    assert!(single_byte_set.all_ascii);
}

