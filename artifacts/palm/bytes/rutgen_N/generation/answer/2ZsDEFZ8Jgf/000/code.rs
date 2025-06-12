// Answer 0

#[test]
fn test_limit_with_exact_bytes() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            TestBuf { limit, data, position: 0 }
        }
        
        fn limit(&self) -> usize {
            self.limit
        }

        fn get_u8(&mut self) -> u8 {
            if self.position < self.limit {
                let byte = self.data[self.position];
                self.position += 1;
                byte
            } else {
                panic!("Attempted to read beyond limit");
            }
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec(), 2);

    assert_eq!(2, buf.limit());
    assert_eq!(b'h', buf.get_u8());
    assert_eq!(1, buf.limit());
}

#[test]
fn test_limit_with_fewer_bytes() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            TestBuf { limit, data, position: 0 }
        }
        
        fn limit(&self) -> usize {
            self.limit
        }

        fn get_u8(&mut self) -> u8 {
            if self.position < self.limit {
                let byte = self.data[self.position];
                self.position += 1;
                byte
            } else {
                panic!("Attempted to read beyond limit");
            }
        }
    }

    let mut buf = TestBuf::new(b"hi".to_vec(), 3);

    assert_eq!(3, buf.limit());
    assert_eq!(b'h', buf.get_u8());
    assert_eq!(2, buf.limit());
}

#[test]
#[should_panic(expected = "Attempted to read beyond limit")]
fn test_limit_panics_on_out_of_bounds() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            TestBuf { limit, data, position: 0 }
        }

        fn limit(&self) -> usize {
            self.limit
        }

        fn get_u8(&mut self) -> u8 {
            if self.position < self.limit {
                let byte = self.data[self.position];
                self.position += 1;
                byte
            } else {
                panic!("Attempted to read beyond limit");
            }
        }
    }

    let mut buf = TestBuf::new(b"abc".to_vec(), 2);

    // consume all allowed bytes
    buf.get_u8();
    buf.get_u8();
    
    // this should panic
    buf.get_u8();
}

