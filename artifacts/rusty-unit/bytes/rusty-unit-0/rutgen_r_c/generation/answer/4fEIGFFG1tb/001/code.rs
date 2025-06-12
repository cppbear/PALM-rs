// Answer 0

#[test]
fn test_from_owner_empty_slice() {
    struct EmptyOwner;

    impl AsRef<[u8]> for EmptyOwner {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    let owner = EmptyOwner;
    let bytes = Bytes::from_owner(owner);

    assert_eq!(bytes.len(), 0);
    assert!(bytes.is_empty());
}

#[test]
fn test_from_owner_non_empty_slice() {
    struct NonEmptyOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for NonEmptyOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner = NonEmptyOwner {
        data: vec![1, 2, 3, 4],
    };
    let bytes = Bytes::from_owner(owner);

    assert_eq!(bytes.len(), 4);
    assert!(!bytes.is_empty());
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_from_owner_panics_on_empty_owner() {
    struct OwnerThatPanics;

    impl AsRef<[u8]> for OwnerThatPanics {
        fn as_ref(&self) -> &[u8] {
            panic!();
        }
    }

    let owner = OwnerThatPanics;
    let _ = Bytes::from_owner(owner); // This should panic when trying to reference the slice
}

#[test]
fn test_from_owner_with_multiple_owners() {
    struct MultiOwner {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for MultiOwner {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let owner1 = MultiOwner {
        data: vec![5, 6, 7],
    };
    let bytes1 = Bytes::from_owner(owner1);

    let owner2 = MultiOwner {
        data: vec![8, 9, 10],
    };
    let bytes2 = Bytes::from_owner(owner2);

    assert_eq!(bytes1.len(), 3);
    assert_eq!(bytes2.len(), 3);
    assert!(!bytes1.is_empty());
    assert!(!bytes2.is_empty());
}

