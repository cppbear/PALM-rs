// Answer 0

#[test]
fn test_split_to_valid_range() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_to(5);
}

#[test]
fn test_split_to_middle() {
    let mut bytes = Bytes::from_static(b"rust programming");
    let result = bytes.split_to(4);
}

#[test]
fn test_split_to_valid_large_input() {
    let input = vec![0u8; 100];
    let mut bytes = Bytes::from_owner(input);
    let result = bytes.split_to(50);
}

#[test]
#[should_panic]
fn test_split_to_exceeds_length() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(8);
}

#[test]
#[should_panic]
fn test_split_to_zero() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(0);
}

