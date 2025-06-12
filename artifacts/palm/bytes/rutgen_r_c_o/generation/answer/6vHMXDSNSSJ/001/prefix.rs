// Answer 0

#[test]
fn test_len_empty() {
    let b = BytesMut::new();
    b.len();
}

#[test]
fn test_len_zeroed() {
    let b = BytesMut::zeroed(0);
    b.len();
}

#[test]
fn test_len_small() {
    let mut b = BytesMut::with_capacity(10);
    b.resize(5, 0);
    b.len();
}

#[test]
fn test_len_full_capacity() {
    let mut b = BytesMut::with_capacity(10);
    b.resize(10, 0);
    b.len();
}

#[test]
fn test_len_exceed_capacity() {
    let mut b = BytesMut::with_capacity(10);
    b.resize(15, 0); // This should not panic as len can be greater than capacity
    b.len();
}

#[test]
fn test_len_large_value() {
    let mut b = BytesMut::with_capacity(usize::MAX >> 5);
    b.resize(usize::MAX >> 5, 0);
    b.len();
}

#[test]
fn test_len_with_data() {
    let mut b = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    b.len();
}

