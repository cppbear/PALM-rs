// Answer 0

#[test]
fn test_into_bytes() {
    struct InnerStruct {
        data: Vec<u8>,
    }

    struct TestStruct {
        inner: InnerStruct,
    }

    impl From<InnerStruct> for Bytes {
        fn from(inner: InnerStruct) -> Self {
            Bytes::from(inner.data)
        }
    }

    let test_instance = TestStruct {
        inner: InnerStruct {
            data: vec![1, 2, 3, 4, 5],
        },
    };

    let bytes = test_instance.into_bytes();
    assert_eq!(bytes, Bytes::from(vec![1, 2, 3, 4, 5]));
}

#[test]
#[should_panic]
fn test_into_bytes_with_empty() {
    struct InnerStruct {
        data: Vec<u8>,
    }

    struct TestStruct {
        inner: InnerStruct,
    }

    impl From<InnerStruct> for Bytes {
        fn from(inner: InnerStruct) -> Self {
            Bytes::from(inner.data)
        }
    }

    let test_instance = TestStruct {
        inner: InnerStruct {
            data: vec![],
        },
    };

    let bytes = test_instance.into_bytes();
    assert_eq!(bytes, Bytes::from(vec![]));
}

#[test]
fn test_into_bytes_with_large_data() {
    struct InnerStruct {
        data: Vec<u8>,
    }

    struct TestStruct {
        inner: InnerStruct,
    }

    impl From<InnerStruct> for Bytes {
        fn from(inner: InnerStruct) -> Self {
            Bytes::from(inner.data)
        }
    }

    let large_data = vec![0; 10_000]; // Large data input
    let test_instance = TestStruct {
        inner: InnerStruct {
            data: large_data,
        },
    };

    let bytes = test_instance.into_bytes();
    assert_eq!(bytes.len(), 10_000);
}

