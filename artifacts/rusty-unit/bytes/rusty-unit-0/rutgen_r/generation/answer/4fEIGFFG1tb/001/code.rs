// Answer 0

#[test]
fn test_from_owner_with_valid_bytes() {
    struct TestOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for TestOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = TestOwner { data: vec![1, 2, 3, 4] };
    let bytes = Bytes::from_owner(owner);
    assert_eq!(bytes.len, 4);
    assert_eq!(unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }, &[1, 2, 3, 4]);
}

#[test]
fn test_from_owner_with_empty_bytes() {
    struct EmptyOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for EmptyOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = EmptyOwner { data: vec![] };
    let bytes = Bytes::from_owner(owner);
    assert_eq!(bytes.len, 0);
    assert_eq!(unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }, &[]);
}

#[test]
#[should_panic]
fn test_from_owner_with_invalid_reference() {
    struct InvalidOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for InvalidOwner {
        fn as_ref(&self) -> &[u8] {
            panic!("Invalid reference");
        }
    }

    let owner = InvalidOwner { data: vec![1, 2, 3] };
    // This should panic due to calling `.as_ref()` on the InvalidOwner
    let _ = Bytes::from_owner(owner);
}

