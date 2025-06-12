// Answer 0

#[test]
fn test_is_empty_on_new_bytesmut() {
    let b = BytesMut::new();
    b.is_empty();
}

#[test]
fn test_is_empty_on_bytesmut_with_zero_capacity() {
    let b = BytesMut::with_capacity(0);
    b.is_empty();
}

#[test]
fn test_is_empty_on_frozen_bytesmut() {
    let mut b = BytesMut::with_capacity(64);
    let frozen_b = b.freeze();
    frozen_b.is_empty();
}

