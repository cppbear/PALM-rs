// Answer 0

#[test]
fn test_copy_to_bytes_with_exact_remaining() {
    struct Inner {
        data: Vec<u8>,
        pos: usize,
    }

    impl Inner {
        fn new(data: Vec<u8>) -> Self {
            Inner { data, pos: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let bytes = crate::Bytes::from(&self.data[self.pos..self.pos + len]);
            self.pos += len;
            bytes
        }
    }

    struct Buffer {
        inner: Inner,
        limit: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Buffer {
                inner: Inner::new(data),
                limit,
            }
        }

        fn remaining(&self) -> usize {
            self.limit.min(self.inner.remaining())
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            assert!(len <= self.remaining(), "`len` greater than remaining");
            let r = self.inner.copy_to_bytes(len);
            self.limit -= len;
            r
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let mut buffer = Buffer::new(data.clone(), data.len());

    let len = buffer.remaining();
    let result = buffer.copy_to_bytes(len);

    assert_eq!(&result[..], &data[..]);
    assert_eq!(buffer.remaining(), 0);
}

