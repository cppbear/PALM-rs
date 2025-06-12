// Answer 0

#[test]
fn test_freeze_empty_bytes_mut() {
    let bytes_mut = BytesMut::new();
    let bytes = bytes_mut.freeze();
    assert_eq!(bytes.len(), 0);
    assert!(bytes.is_empty());
}

#[test]
fn test_freeze_non_empty_bytes_mut() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4]);
    let bytes = bytes_mut.freeze();
    assert_eq!(bytes.len(), 4);
    assert!(!bytes.is_empty());
}

#[test]
fn test_freeze_with_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(4);
    bytes_mut.extend_from_slice(&[5; 4]);
    let bytes = bytes_mut.freeze();
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes.slice(0..4).as_ref(), &[5, 5, 5, 5]);
}

#[test]
fn test_freeze_and_clone() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.extend_from_slice(b"hello world");
    let bytes = bytes_mut.freeze();
    let cloned_bytes = bytes.clone();
    assert_eq!(&cloned_bytes[..], b"hello world");
}

#[test]
fn test_freeze_and_thread_safety() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.extend_from_slice(b"hello world");
    let bytes = bytes_mut.freeze();
    let bytes_clone = bytes.clone();

    let handle = std::thread::spawn(move || {
        assert_eq!(&bytes[..], b"hello world");
    });
    
    assert_eq!(&bytes_clone[..], b"hello world");
    handle.join().unwrap();
}

