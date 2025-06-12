// Answer 0

#[test]
fn test_get_ref() {
    struct MockBuf {
        inner: Vec<u8>,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { inner: data }
        }
        
        fn remaining(&self) -> usize {
            self.inner.len()
        }
    }

    struct TestTake<T> {
        inner: T,
    }

    impl<T> TestTake<T> {
        fn new(inner: T) -> Self {
            TestTake { inner }
        }

        pub fn get_ref(&self) -> &T {
            &self.inner
        }
    }

    let mock_buf = MockBuf::new(b"hello world".to_vec());
    let buf = TestTake::new(mock_buf);

    assert_eq!(11, buf.get_ref().remaining());
}

