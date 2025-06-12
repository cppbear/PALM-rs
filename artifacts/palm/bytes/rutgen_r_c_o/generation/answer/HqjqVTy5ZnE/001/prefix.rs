// Answer 0

#[test]
fn test_split_to_small_size() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_to(1);
}

#[test]
fn test_split_to_medium_size() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_to(3);
}

#[test]
fn test_split_to_large_size() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_to(6);
}

#[test]
fn test_split_to_exact_capacity() {
    let mut b1 = BytesMut::with_capacity(17);
    b1.extend_from_slice(b"hello world!");
    b1.split_to(6);
}

#[test]
#[should_panic]
fn test_split_to_exceeding_size() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_to(7);
}

#[test]
fn test_split_to_min_capacity() {
    let mut b1 = BytesMut::with_capacity(10);
    b1.extend_from_slice(b"hello");
    b1.split_to(5);
}

