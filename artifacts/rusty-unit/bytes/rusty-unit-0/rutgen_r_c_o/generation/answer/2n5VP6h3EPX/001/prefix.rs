// Answer 0

#[test]
fn test_split_empty() {
    let mut buf = BytesMut::new();
    let other = buf.split();
}

#[test]
fn test_split_single_byte() {
    let mut buf = BytesMut::with_capacity(1);
    buf.extend_from_slice(&[42]);
    let other = buf.split();
}

#[test]
fn test_split_multiple_bytes() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    let other = buf.split();
}

#[test]
fn test_split_non_empty_with_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    let other = buf.split();
}

#[test]
fn test_split_full_capacity() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.resize(1024, 0);
    let other = buf.split();
}

#[test]
#[should_panic]
fn test_split_panic_on_empty() {
    let mut buf = BytesMut::new();
    let _other = buf.split(); // Must not panic
}

#[test]
fn test_split_after_truncate() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    buf.truncate(3);
    let other = buf.split();
}

#[test]
fn test_split_large_capacity() {
    let mut buf = BytesMut::with_capacity(usize::MAX);
    let other = buf.split();
}

