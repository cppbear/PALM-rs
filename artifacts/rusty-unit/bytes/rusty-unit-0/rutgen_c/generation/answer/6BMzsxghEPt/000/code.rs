// Answer 0

#[test]
fn test_from_iter_empty() {
    let bytes: Bytes = Bytes::from_iter([]);
    assert_eq!(bytes.len, 0);
}

#[test]
fn test_from_iter_single_element() {
    let bytes: Bytes = Bytes::from_iter([42]);
    assert_eq!(bytes.len, 1);
    unsafe {
        assert_eq!(*bytes.ptr, 42);
    }
}

#[test]
fn test_from_iter_multiple_elements() {
    let bytes: Bytes = Bytes::from_iter([1, 2, 3, 4, 5]);
    assert_eq!(bytes.len, 5);
    unsafe {
        let slice = std::slice::from_raw_parts(bytes.ptr, bytes.len);
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_from_iter_large_collection() {
    let collection = (0..100).collect::<Vec<u8>>();
    let bytes: Bytes = Bytes::from_iter(collection);
    assert_eq!(bytes.len, 100);
    unsafe {
        let slice = std::slice::from_raw_parts(bytes.ptr, bytes.len);
        assert_eq!(slice.to_vec(), (0..100).collect::<Vec<u8>>());
    }
}

