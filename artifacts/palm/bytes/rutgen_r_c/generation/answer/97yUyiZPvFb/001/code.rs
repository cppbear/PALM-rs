// Answer 0

#[test]
fn test_last_mut() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
            if self.position > self.data.len() {
                panic!("Buffer overflow");
            }
        }

        fn copy_to_bytes(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf_a = TestBuf::new(b"hello ".to_vec());
    let mut buf_b = TestBuf::new(b"world".to_vec());

    let mut chain = Chain::new(buf_a, buf_b);

    // Testing mutable access to the last buffer
    {
        let last_buf = chain.last_mut();
        last_buf.advance(1); // Should not panic
    }

    // Check the behavior of the overall chain
    let full = chain.last_mut().copy_to_bytes();
    assert_eq!(full, b"orld"[..]);
}

#[test]
#[should_panic(expected = "Buffer overflow")]
fn test_last_mut_overflow() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
            if self.position > self.data.len() {
                panic!("Buffer overflow");
            }
        }
    }

    let mut buf_a = TestBuf::new(b"hello ".to_vec());
    let mut buf_b = TestBuf::new(b"world".to_vec());

    let mut chain = Chain::new(buf_a, buf_b);

    // This should trigger a panic due to buffer overflow
    let last_buf = chain.last_mut();
    last_buf.advance(10); // Will cause panic
}

