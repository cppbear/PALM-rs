// Answer 0

#[test]
fn test_find_empty_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    let result = single_byte_set.find(b"sample text");
    assert_eq!(result, None);
}

#[test]
fn test_find_single_byte_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],
        complete: false,
        all_ascii: false,
    };
    let result = single_byte_set.find(b"sample text");
    assert_eq!(result, None); // 'a' not present
}

#[test]
fn test_find_two_byte_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: false,
    };
    let result = single_byte_set.find(b"sample text");
    assert_eq!(result, None); // 'a' and 'b' not present
}

#[test]
fn test_find_three_byte_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: false,
    };
    let result = single_byte_set.find(b"sample text");
    assert_eq!(result, None); // 'a', 'b', and 'c' not present
}

