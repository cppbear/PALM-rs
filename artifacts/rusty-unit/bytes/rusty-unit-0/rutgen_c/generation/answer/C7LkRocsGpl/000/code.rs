// Answer 0

#[test]
fn test_remaining() {
    struct TestBuf {
        bytes: Bytes,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf {
                bytes: Bytes::from_static(data),
            }
        }
    }

    let buf = TestBuf::new(&[1, 2, 3, 4]);
    assert_eq!(buf.bytes.remaining(), 4);
}

#[test]
fn test_remaining_empty() {
    struct TestBuf {
        bytes: Bytes,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf {
                bytes: Bytes::from_static(data),
            }
        }
    }

    let buf = TestBuf::new(&[]);
    assert_eq!(buf.bytes.remaining(), 0);
}

