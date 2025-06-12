// Answer 0

#[test]
fn test_split_to_at_len() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(bytes.len());
}

#[test]
fn test_split_to_at_zero() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(0);
}

#[test]
#[should_panic]
fn test_split_to_at_exceeds_len() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(8);
}

#[test]
fn test_split_to_at_mid_point() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(4);
} 

#[test]
fn test_split_to_at_half_len() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(bytes.len() / 2);
} 

#[test]
fn test_split_to_at_one_less_than_len() {
    let mut bytes = Bytes::from_static(b"example");
    let result = bytes.split_to(bytes.len() - 1);
} 

#[test]
fn test_split_to_at_full_slice() {
    let mut bytes = Bytes::from_static(b"");
    let result = bytes.split_to(0);
}

