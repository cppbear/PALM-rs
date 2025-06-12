// Answer 0

#[test]
fn test_split_off_at_zero() {
    let mut bytes = Bytes::from_static(&b"hello"[..]);
    let b = bytes.split_off(0);
}

#[test]
fn test_split_off_at_length() {
    let mut bytes = Bytes::from_static(&b"hello"[..]);
    let b = bytes.split_off(5);
}

#[test]
fn test_split_off_within_bounds() {
    let mut bytes = Bytes::from_static(&b"hello"[..]);
    let b = bytes.split_off(3);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::from_static(&b"hello"[..]);
    let b = bytes.split_off(6);
}

