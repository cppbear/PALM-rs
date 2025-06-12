// Answer 0

#[test]
fn test_get_ref() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new(data: Vec<u8>) -> Self {
            MockBufMut { data }
        }
    }

    struct BufferWrapper<T> {
        inner: T,
    }

    impl BufferWrapper<MockBufMut> {
        fn new(inner: MockBufMut) -> Self {
            BufferWrapper { inner }
        }

        fn get_ref(&self) -> &MockBufMut {
            &self.inner
        }
    }

    let mock_buf = MockBufMut::new(vec![1, 2, 3, 4]);
    let buffer_wrapper = BufferWrapper::new(mock_buf);
    let result = buffer_wrapper.get_ref();

    assert_eq!(result.data, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_get_ref_panic() {
    struct EmptyBufMut;

    struct BufferWrapper<T> {
        inner: T,
    }

    impl BufferWrapper<EmptyBufMut> {
        fn new(inner: EmptyBufMut) -> Self {
            BufferWrapper { inner }
        }

        fn get_ref(&self) -> &EmptyBufMut {
            panic!("Attempted to access an uninitialized buffer.");
        }
    }

    let empty_buf = EmptyBufMut;
    let buffer_wrapper = BufferWrapper::new(empty_buf);
    buffer_wrapper.get_ref();
}

