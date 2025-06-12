// Answer 0

#[test]
fn test_byte_str_new() {
    struct ByteStr {
        bytes: Bytes,
    }

    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn new() -> Self {
            Bytes { data: Vec::new() }
        }
    }

    impl ByteStr {
        fn new() -> Self {
            ByteStr {
                bytes: Bytes::new(),
            }
        }
    }

    let byte_str = ByteStr::new();
    assert!(byte_str.bytes.data.is_empty());
}

