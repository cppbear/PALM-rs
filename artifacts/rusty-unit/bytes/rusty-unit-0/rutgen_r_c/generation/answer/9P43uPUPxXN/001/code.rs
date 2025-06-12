// Answer 0

#[test]
fn test_is_empty_with_new() {
    let b = BytesMut::new();
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_with_capacity() {
    let b = BytesMut::with_capacity(64);
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_after_truncate() {
    let mut b = BytesMut::with_capacity(64);
    b.truncate(0);
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut b = BytesMut::with_capacity(64);
    b.clear();
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_after_resize_to_zero() {
    let mut b = BytesMut::with_capacity(64);
    b.resize(0, 0);
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_after_split() {
    let mut b = BytesMut::with_capacity(64);
    let split = b.split();
    assert!(split.is_empty());
}

