// Answer 0

#[test]
fn test_put_bytes_zero_count() {
    let mut buf: Vec<u8> = Vec::new();
    buf.put_bytes(100, 0);
}

#[test]
fn test_put_bytes_small_value() {
    let mut buf: Vec<u8> = Vec::new();
    buf.put_bytes(50, 5);
}

#[test]
fn test_put_bytes_large_value() {
    let mut buf: Vec<u8> = Vec::new();
    buf.put_bytes(255, 10);
}

#[test]
fn test_put_bytes_edge_length() {
    let mut buf: Vec<u8> = Vec::with_capacity(usize::MAX - 10);
    buf.resize(usize::MAX - 10, 0);
    buf.put_bytes(20, 10);
}

#[test]
#[should_panic]
fn test_put_bytes_exceed_capacity() {
    let mut buf: Vec<u8> = Vec::with_capacity(usize::MAX);
    buf.resize(usize::MAX, 0);
    buf.put_bytes(1, 1);
}

#[test]
fn test_put_bytes_non_zero_initial() {
    let mut buf: Vec<u8> = vec![1, 2, 3];
    buf.put_bytes(0, 3);
}

#[test]
fn test_put_bytes_increase_size() {
    let mut buf: Vec<u8> = vec![1, 2, 3];
    buf.put_bytes(42, 5);
}

