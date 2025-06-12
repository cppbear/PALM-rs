// Answer 0

#[test]
fn test_remaining_non_empty() {
    let bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5) };
    assert_eq!(bytes_mut.remaining(), 5);
}

#[test]
fn test_remaining_empty() {
    let bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.remaining(), 0);
}

#[test]
fn test_remaining_after_truncate() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe { bytes_mut.set_len(15) };
    bytes_mut.truncate(10);
    assert_eq!(bytes_mut.remaining(), 10);
}

#[test]
fn test_remaining_after_resize() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe { bytes_mut.set_len(10) };
    bytes_mut.resize(5, 0);
    assert_eq!(bytes_mut.remaining(), 5);
}

#[test]
fn test_remaining_after_split() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe { bytes_mut.set_len(10) };
    let split_bytes = bytes_mut.split();
    assert_eq!(split_bytes.remaining(), 10);
}

