// Answer 0

#[test]
fn test_find_empty_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    let result = single_byte_set.find(b"test string");
}

#[test]
fn test_find_empty_text() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    let result = single_byte_set.find(b"");
}

