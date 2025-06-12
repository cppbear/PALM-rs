// Answer 0

#[test]
fn test_clear_empty_buffer() {
    let mut buf = BytesMut::new();
    buf.clear();
}

#[test]
fn test_clear_non_empty_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 1);
    buf.clear();
}

#[test]
fn test_clear_full_capacity_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 1);
    buf.clear();
}

#[test]
fn test_clear_zeroed_buffer() {
    let mut buf = BytesMut::zeroed(5);
    buf.clear();
}

#[test]
fn test_clear_buffer_with_resolution_in_between() {
    let mut buf = BytesMut::with_capacity(20);
    buf.resize(15, 1);
    buf.clear();
}

