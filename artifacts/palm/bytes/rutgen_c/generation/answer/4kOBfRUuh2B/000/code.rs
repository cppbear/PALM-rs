// Answer 0

#[test]
fn test_get_mut() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }
    
    impl core::ops::Deref for MockBuf {
        type Target = Vec<u8>;

        fn deref(&self) -> &Self::Target {
            &self.data[self.position..]
        }
    }

    let mut buf = Take {
        inner: MockBuf::new(b"hello world".to_vec()),
        limit: 2,
    };
    
    let inner_buf = buf.get_mut();
    inner_buf.advance(2);
    
    let mut dst = vec![];
    dst.extend_from_slice(&inner_buf.data[inner_buf.position..]); 
    
    assert_eq!(dst, b"ll"[..]);
}

#[test]
fn test_get_mut_with_empty_buffer() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }
    
    impl core::ops::Deref for MockBuf {
        type Target = Vec<u8>;

        fn deref(&self) -> &Self::Target {
            &self.data[self.position..]
        }
    }

    let mut buf = Take {
        inner: MockBuf::new(Vec::new()),
        limit: 2,
    };

    let inner_buf = buf.get_mut();
    assert!(inner_buf.remaining() == 0);
}

#[test]
fn test_get_mut_boundary_condition() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }
    
    impl core::ops::Deref for MockBuf {
        type Target = Vec<u8>;

        fn deref(&self) -> &Self::Target {
            &self.data[self.position..]
        }
    }

    let mut buf = Take {
        inner: MockBuf::new(b"abcd".to_vec()),
        limit: 4,
    };
    
    let inner_buf = buf.get_mut();
    inner_buf.advance(4);
    
    assert_eq!(inner_buf.remaining(), 0);
}

