// Answer 0

#[test]
fn test_bytes_new() {
    let b = Bytes::new();
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
}

#[test]
fn test_bytes_new_static() {
    const DATA: &[u8] = b"Hello, world!";
    let b = Bytes::from_static(DATA);
    assert_eq!(b.len(), DATA.len());
    assert!(!b.is_empty());
}

#[test]
fn test_bytes_slice_empty() {
    let b = Bytes::new();
    let slice = b.slice(0..0);
    assert_eq!(slice.len(), 0);
    assert!(slice.is_empty());
}

#[test]
fn test_bytes_slice_full() {
    const DATA: &[u8] = b"Sample";
    let b = Bytes::from_static(DATA);
    let slice = b.slice(0..DATA.len());
    assert_eq!(slice.len(), DATA.len());
    assert!(!slice.is_empty());
}

#[test]
#[should_panic]
fn test_bytes_slice_out_of_bounds() {
    const DATA: &[u8] = b"Test";
    let b = Bytes::from_static(DATA);
    let _slice = b.slice(5..6); // Should panic
}

#[test]
#[should_panic]
fn test_bytes_slice_start_greater_than_end() {
    const DATA: &[u8] = b"Test";
    let b = Bytes::from_static(DATA);
    let _slice = b.slice(2..1); // Should panic
}

