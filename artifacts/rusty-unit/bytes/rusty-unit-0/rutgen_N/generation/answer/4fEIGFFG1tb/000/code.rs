// Answer 0

#[test]
fn test_from_owner_with_valid_data() {
    struct ValidOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for ValidOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = ValidOwner {
        data: vec![1, 2, 3, 4],
    };

    let bytes = from_owner(owner);
    assert_eq!(bytes.len, 4);
    assert_eq!(unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }, &[1, 2, 3, 4]);
}

#[test]
fn test_from_owner_empty_data() {
    struct EmptyOwner;

    impl AsRef<[u8]> for EmptyOwner {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    let owner = EmptyOwner;
    let bytes = from_owner(owner);
    assert_eq!(bytes.len, 0);
    assert_eq!(unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }, &[]);
}

#[should_panic]
fn test_from_owner_with_invalid_data() {
    struct InvalidOwner;

    impl AsRef<[u8]> for InvalidOwner {
        fn as_ref(&self) -> &[u8] {
            panic!("Simulated panic");
        }
    }

    let owner = InvalidOwner;
    let _bytes = from_owner(owner); // This should panic
}

