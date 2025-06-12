// Answer 0

#[test]
fn test_split_off_zero() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(0);
}

#[test]
fn test_split_off_middle() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(5);
}

#[test]
fn test_split_off_end() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_off(5);
}

#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_off(6);
}

