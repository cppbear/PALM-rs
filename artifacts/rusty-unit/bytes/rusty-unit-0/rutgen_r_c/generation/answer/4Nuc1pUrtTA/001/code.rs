// Answer 0

#[test]
fn test_try_into_mut_unique() {
    use alloc::boxed::Box;

    struct DummyOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for DummyOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = DummyOwner { data: b"hello".to_vec() };
    let bytes = Bytes::from_owner(owner);
    assert!(bytes.is_unique());
    
    match bytes.try_into_mut() {
        Ok(bytes_mut) => {
            assert_eq!(bytes_mut.as_slice(), b"hello");
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
#[should_panic]
fn test_try_into_mut_not_unique() {
    struct DummyOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for DummyOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = DummyOwner { data: b"hello".to_vec() };
    let bytes_shared = Bytes::from_static(b"hello");

    let result = bytes_shared.try_into_mut();
    assert!(result.is_err());
}

