// Answer 0

#[test]
fn test_from_owner_empty_slice() {
    struct OwnerEmpty;

    impl AsRef<[u8]> for OwnerEmpty {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    let owner = OwnerEmpty;
    let bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_small_slice() {
    struct OwnerSmall;

    impl AsRef<[u8]> for OwnerSmall {
        fn as_ref(&self) -> &[u8] {
            b"small"
        }
    }

    let owner = OwnerSmall;
    let bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_large_slice() {
    struct OwnerLarge;

    impl AsRef<[u8]> for OwnerLarge {
        fn as_ref(&self) -> &[u8] {
            b"this is a considerably larger buffer to test the from_owner functionality"
        }
    }

    let owner = OwnerLarge;
    let bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_multiple_bytes() {
    struct OwnerMultiple;

    impl AsRef<[u8]> for OwnerMultiple {
        fn as_ref(&self) -> &[u8] {
            b"multiple bytes are definitely better than one"
        }
    }

    let owner = OwnerMultiple;
    let bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_non_ascii() {
    struct OwnerNonAscii;

    impl AsRef<[u8]> for OwnerNonAscii {
        fn as_ref(&self) -> &[u8] {
            b"\xF0\x9F\x98\x81" // U+1F601 Grinning Face with Smiling Eyes
        }
    }

    let owner = OwnerNonAscii;
    let bytes = Bytes::from_owner(owner);
}

#[test]
fn test_from_owner_max_size() {
    struct OwnerMaxSize;

    impl AsRef<[u8]> for OwnerMaxSize {
        fn as_ref(&self) -> &[u8] {
            vec![0; usize::MAX].as_slice()
        }
    }

    let owner = OwnerMaxSize;
    let bytes = Bytes::from_owner(owner);
}

