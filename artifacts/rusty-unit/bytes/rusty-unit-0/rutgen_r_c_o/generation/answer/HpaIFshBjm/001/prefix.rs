// Answer 0

#[test]
fn test_truncate_exact_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0u8);
    buf.truncate(buf.len());
}

#[test]
fn test_truncate_non_empty() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let original_len = buf.len();
    buf.truncate(original_len);
}

#[test]
fn test_truncate_edge_length() {
    let mut buf = BytesMut::from_vec(vec![10, 20, 30, 40, 50]);
    buf.truncate(0);
}

#[test]
fn test_truncate_max_length() {
    let mut buf = BytesMut::from_vec(vec![100, 101, 102, 103, 104]);
    buf.truncate(5);
}

#[test]
fn test_truncate_less_than_length() {
    let mut buf = BytesMut::from_vec(vec![5, 4, 3, 2, 1]);
    buf.truncate(3);
}

