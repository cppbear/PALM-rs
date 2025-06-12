// Answer 0

#[test]
fn test_get_ref_with_non_empty_buf() {
    struct TestBuf {
        inner: Vec<u8>,
    }
    
    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { inner: data }
        }
        
        fn remaining(&self) -> usize {
            self.inner.len()
        }
    }

    let buf = TestBuf::new(b"hello world".to_vec());
    assert_eq!(buf.remaining(), 11);
    let ref_inner = &buf.inner;
    assert_eq!(ref_inner, b"hello world");
}

#[test]
fn test_get_ref_with_empty_buf() {
    struct TestBuf {
        inner: Vec<u8>,
    }
    
    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { inner: data }
        }
        
        fn remaining(&self) -> usize {
            self.inner.len()
        }
    }

    let buf = TestBuf::new(Vec::new());
    assert_eq!(buf.remaining(), 0);
    let ref_inner = &buf.inner;
    assert_eq!(ref_inner.len(), 0);
}

