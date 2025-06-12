// Answer 0

#[test]
fn test_split_off_at_zero() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(0);
}

#[test]
fn test_split_off_at_one() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(1);
}

#[test]
fn test_split_off_at_midpoint() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(5);
}

#[test]
fn test_split_off_at_len_minus_one() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(10);
}

#[test]
#[should_panic]
fn test_split_off_at_len() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(11);
}

