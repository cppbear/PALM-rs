// Answer 0

#[test]
fn test_find_empty_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.find(b"hello"), None);
}

#[test]
fn test_find_one_byte_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.find(b"hello"), None);
    assert_eq!(single_byte_set.find(b"apple"), Some(1));
}

#[test]
fn test_find_two_bytes_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'e'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.find(b"hello"), Some(1));
    assert_eq!(single_byte_set.find(b"cat"), None);
}

#[test]
fn test_find_three_bytes_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'e', b'o'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.find(b"hello"), Some(4));
    assert_eq!(single_byte_set.find(b"cat"), None);
}

#[test]
fn test_find_more_than_three_bytes_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c', b'd'],
        complete: false,
        all_ascii: false,
    };
    assert_eq!(single_byte_set.find(b"hello"), None);
    assert_eq!(single_byte_set.find(b"abcde"), Some(0));
}

