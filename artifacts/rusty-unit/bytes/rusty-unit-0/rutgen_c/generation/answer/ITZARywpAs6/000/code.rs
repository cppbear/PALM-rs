// Answer 0

#[test]
fn test_get_ref() {
    struct MockBufMut {
        value: Vec<u8>,
    }
    
    impl MockBufMut {
        fn new() -> Self {
            MockBufMut { value: vec![1, 2, 3] }
        }
    }
    
    let mock_buf = MockBufMut::new();
    let limit_buf = Limit { inner: mock_buf, limit: 10 };
    
    let reference = limit_buf.get_ref();
    
    assert_eq!(reference.value, vec![1, 2, 3]);
}

#[test]
fn test_get_ref_empty() {
    struct MockBufMut {
        value: Vec<u8>,
    }
    
    impl MockBufMut {
        fn new() -> Self {
            MockBufMut { value: vec![] }
        }
    }
    
    let mock_buf = MockBufMut::new();
    let limit_buf = Limit { inner: mock_buf, limit: 0 };
    
    let reference = limit_buf.get_ref();
    
    assert_eq!(reference.value, vec![]);
}

