// Answer 0

#[test]
fn test_copy_to_bytes_valid() {
    struct Inner {
        data: Vec<u8>,
        position: usize,
    }

    impl Inner {
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let bytes = self.data[self.position..self.position + len].to_vec();
            self.position += len;
            crate::Bytes::from(bytes)
        }
    }

    struct Buffer {
        inner: Inner,
        limit: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.limit
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            assert!(len <= self.remaining(), "`len` greater than remaining");
            let r = self.inner.copy_to_bytes(len);
            self.limit -= len;
            r
        }
    }

    let mut buffer = Buffer {
        inner: Inner {
            data: vec![1, 2, 3, 4, 5],
            position: 0,
        },
        limit: 5,
    };
    
    let bytes = buffer.copy_to_bytes(3);
    assert_eq!(buffer.limit, 2);
    assert_eq!(bytes.to_vec(), vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "`len` greater than remaining")]
fn test_copy_to_bytes_over_limit() {
    struct Inner {
        data: Vec<u8>,
        position: usize,
    }

    impl Inner {
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let bytes = self.data[self.position..self.position + len].to_vec();
            self.position += len;
            crate::Bytes::from(bytes)
        }
    }

    struct Buffer {
        inner: Inner,
        limit: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.limit
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            assert!(len <= self.remaining(), "`len` greater than remaining");
            let r = self.inner.copy_to_bytes(len);
            self.limit -= len;
            r
        }
    }

    let mut buffer = Buffer {
        inner: Inner {
            data: vec![1, 2, 3, 4, 5],
            position: 0,
        },
        limit: 2,
    };
    
    let _ = buffer.copy_to_bytes(3);
}

