// Answer 0

#[test]
fn test_with_capacity_zero() {
    let bytes = BytesMut::with_capacity(0);
}

#[test]
fn test_with_capacity_small() {
    let bytes = BytesMut::with_capacity(1);
}

#[test]
fn test_with_capacity_medium() {
    let bytes = BytesMut::with_capacity(16);
}

#[test]
fn test_with_capacity_large() {
    let bytes = BytesMut::with_capacity(1024);
}

#[test]
fn test_with_capacity_max() {
    let bytes = BytesMut::with_capacity(usize::MAX);
}

