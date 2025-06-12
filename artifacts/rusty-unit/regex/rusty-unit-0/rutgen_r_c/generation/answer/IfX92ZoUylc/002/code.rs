// Answer 0

#[test]
fn test_find_with_three_dense_items_found() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: true,
    };
    let text = b"abcde";
    assert_eq!(single_byte_set.find(text), Some(0));
}

#[test]
fn test_find_with_three_dense_items_not_found() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: true,
    };
    let text = b"xyz";
    assert_eq!(single_byte_set.find(text), None);
}

#[test]
fn test_find_with_three_dense_items_found_in_larger_text() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'x', b'y', b'z'],
        complete: false,
        all_ascii: true,
    };
    let text = b"abcdefghijxyzklmnop";
    assert_eq!(single_byte_set.find(text), Some(10));
}

#[test]
fn test_find_with_empty_dense() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    let text = b"anytext";
    assert_eq!(single_byte_set.find(text), None);
}

