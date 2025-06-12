// Answer 0

#[test]
fn test_limit_initial_value() {
    struct TestBuf {
        data: Vec<u8>,
        positioned: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, positioned: 0 }
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.positioned];
            self.positioned += 1;
            byte
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.positioned
        }
    }

    let inner_buf = TestBuf::new(b"hello world".to_vec());
    let mut take = Take { inner: inner_buf, limit: 2 };

    assert_eq!(2, take.limit());
}

#[test]
fn test_limit_after_read() {
    struct TestBuf {
        data: Vec<u8>,
        positioned: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, positioned: 0 }
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.positioned];
            self.positioned += 1;
            byte
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.positioned
        }
    }

    let inner_buf = TestBuf::new(b"hello world".to_vec());
    let mut take = Take { inner: inner_buf, limit: 2 };

    assert_eq!(2, take.limit());
    let _ = take.inner.get_u8();
    assert_eq!(1, take.limit());
}

#[test]
fn test_limit_cannot_exceed_size() {
    struct TestBuf {
        data: Vec<u8>,
        positioned: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, positioned: 0 }
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.positioned];
            self.positioned += 1;
            byte
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.positioned
        }
    }

    let inner_buf = TestBuf::new(b"hi".to_vec());
    let mut take = Take { inner: inner_buf, limit: 3 };

    assert_eq!(2, take.inner.remaining());
    let _ = take.inner.get_u8();
    let _ = take.inner.get_u8();
    
    assert_eq!(0, take.limit());
}

