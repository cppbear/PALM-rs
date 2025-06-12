// Answer 0

#[test]
fn test_len_empty() {
    let b = Bytes::new();
    let _ = b.len();
}

#[test]
fn test_len_static_small() {
    let b = Bytes::from_static(&[1, 2, 3]);
    let _ = b.len();
}

#[test]
fn test_len_static_large() {
    let b = Bytes::from_static(&[0; usize::MAX]);
    let _ = b.len();
}

#[test]
fn test_len_from_owner() {
    let owner = vec![4, 5, 6];
    let b = Bytes::from_owner(owner);
    let _ = b.len();
}

#[test]
fn test_len_from_slice() {
    let data: &[u8] = b"Test data";
    let b = Bytes::copy_from_slice(data);
    let _ = b.len();
}

