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

    let result = new();
    
    assert_eq!(result.sparse.len(), 256);
    for &value in &result.sparse {
        assert_eq!(value, false);
    }
    assert!(result.dense.is_empty());
    assert!(result.complete);
    assert!(result.all_ascii);
}

