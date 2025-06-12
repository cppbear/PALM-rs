// Answer 0

#[test]
fn test_find_empty_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    single_byte_set.find(b"test text");
}

#[test]
fn test_find_one_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],
        complete: false,
        all_ascii: true,
    };
    single_byte_set.find(b"test text with a");
}

#[test]
fn test_find_two_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: true,
    };
    single_byte_set.find(b"test text with ab");
}

#[test]
fn test_find_three_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: true,
    };
    single_byte_set.find(b"test text with abc");
}

#[test]
fn test_find_four_plus_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c', b'd', b'e', b'f'],
        complete: false,
        all_ascii: true,
    };
    single_byte_set.find(b"test text with abcdef");
}

