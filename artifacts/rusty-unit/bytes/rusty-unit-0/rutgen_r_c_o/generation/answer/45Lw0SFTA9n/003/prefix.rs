// Answer 0

#[test]
fn test_split_to_zero_length() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(0);
}

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut a = Bytes::from_static(b"hello");
    let _ = a.split_to(6);
}

#[test]
fn test_split_to_empty_bytes() {
    let mut a = Bytes::new();
    let b = a.split_to(0);
}

#[test]
fn test_split_to_non_zero() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(3);
}

#[test]
fn test_split_to_full_length() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(5);
}

