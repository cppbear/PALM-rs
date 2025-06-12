// Answer 0

#[test]
fn test_find_single_byte() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],  // valid single byte
        complete: false,
        all_ascii: true,
    };
    let text = b"hello, world!";
    let result = single_byte_set.find(text);
    assert_eq!(result, Some(4)); // 'a' is at index 4 in "hello, world!"
}

#[test]
fn test_find_single_byte_not_found() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'z'],  // valid single byte
        complete: false,
        all_ascii: true,
    };
    let text = b"hello, world!";
    let result = single_byte_set.find(text);
    assert_eq!(result, None); // 'z' is not in "hello, world!"
}

#[test]
fn test_find_single_byte_empty_text() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],  // valid single byte
        complete: false,
        all_ascii: true,
    };
    let text = b"";  // empty text
    let result = single_byte_set.find(text);
    assert_eq!(result, None); // no matches in empty text
}

#[test]
#[should_panic]
fn test_find_single_byte_panic_condition() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],  // invalid since dense is empty
        complete: false,
        all_ascii: true,
    };
    let text = b"hello, world!";
    let _result = single_byte_set.find(text); // should panic when trying to access dense[0]
}

