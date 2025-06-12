// Answer 0

#[test]
fn test_into_inner_with_non_empty_buf() {
    struct MockBuf {
        data: Vec<u8>,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { data }
        }
        
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    struct MockReader {
        buf: MockBuf,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader {
                buf: MockBuf::new(data),
            }
        }

        fn into_inner(self) -> MockBuf {
            self.buf
        }
    }

    let reader = MockReader::new(vec![1, 2, 3, 4, 5]);
    let buf = reader.into_inner();
    assert_eq!(buf.remaining(), 5);
}

#[test]
fn test_into_inner_with_empty_buf() {
    struct MockBuf {
        data: Vec<u8>,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { data }
        }
        
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    struct MockReader {
        buf: MockBuf,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader {
                buf: MockBuf::new(data),
            }
        }

        fn into_inner(self) -> MockBuf {
            self.buf
        }
    }

    let reader = MockReader::new(Vec::new());
    let buf = reader.into_inner();
    assert_eq!(buf.remaining(), 0);
}

