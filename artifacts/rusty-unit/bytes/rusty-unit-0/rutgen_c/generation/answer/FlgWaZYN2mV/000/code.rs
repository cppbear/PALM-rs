// Answer 0

#[test]
fn test_split_off_empty_bytes() {
    let mut bytes = Bytes::new();
    let new_bytes = bytes.split_off(0);
    assert!(bytes.is_empty());
    assert!(new_bytes.is_empty());
}

#[test]
fn test_split_off_single_element() {
    let input = b"hello";
    let mut bytes = Bytes::from_static(input);
    let new_bytes = bytes.split_off(1);
    assert_eq!(bytes.as_slice(), &input[..1]);
    assert_eq!(new_bytes.as_slice(), &input[1..]);
}

#[test]
fn test_split_off_all_elements() {
    let input = b"hello";
    let mut bytes = Bytes::from_static(input);
    let new_bytes = bytes.split_off(5);
    assert!(bytes.is_empty());
    assert_eq!(new_bytes.as_slice(), input);
}

#[should_panic]
fn test_split_off_out_of_bounds() {
    let input = b"hello";
    let mut bytes = Bytes::from_static(input);
    bytes.split_off(6); // Out of bounds, should panic
}

#[test]
fn test_split_off_first_element() {
    let input = b"hello";
    let mut bytes = Bytes::from_static(input);
    let new_bytes = bytes.split_off(0);
    assert!(bytes.is_empty());
    assert_eq!(new_bytes.as_slice(), input);
}

