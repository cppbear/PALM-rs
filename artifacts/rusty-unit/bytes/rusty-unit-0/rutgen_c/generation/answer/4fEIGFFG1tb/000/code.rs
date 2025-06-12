// Answer 0

#[test]
fn test_from_owner_with_vec() {
    let owner = Vec::from("hello".as_bytes());
    let bytes = Bytes::from_owner(owner);
    
    assert_eq!(bytes.len(), 5);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, b"hello");
}

#[test]
fn test_from_owner_with_string() {
    let owner = String::from("world");
    let bytes = Bytes::from_owner(owner);
    
    assert_eq!(bytes.len(), 5);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, b"world");
}

#[test]
fn test_from_owner_with_empty_slice() {
    let owner: &[u8] = &[];
    let bytes = Bytes::from_owner(owner);
    
    assert_eq!(bytes.len(), 0);
    assert!(bytes.is_empty());
}

#[test]
#[should_panic]
fn test_from_owner_with_null_pointer() {
    struct InvalidOwner;
    
    impl AsRef<[u8]> for InvalidOwner {
        fn as_ref(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(std::ptr::null(), 1) } // unsafe, will panic
        }
    }

    let owner = InvalidOwner;
    let _bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_with_cloned_vec() {
    let original = Vec::from("test".as_bytes());
    let bytes = Bytes::from_owner(original.clone());
    
    assert_eq!(bytes.len(), 4);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, b"test");
}

