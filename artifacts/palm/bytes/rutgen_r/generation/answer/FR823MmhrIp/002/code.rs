// Answer 0

#[test]
fn test_copy_to_bytes_len_greater_than_remaining() {
    struct Inner {
        data: Vec<u8>,
        position: usize,
    }

    impl Inner {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let result = crate::Bytes::from(&self.data[self.position..self.position + len]);
            self.position += len;
            result
        }
    }

    struct Buffer {
        inner: Inner,
        limit: usize,
    }

    impl Buffer {
        fn new(inner: Inner, limit: usize) -> Self {
            Self { inner, limit }
        }

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

    let data = vec![1, 2, 3, 4, 5];
    let mut buffer = Buffer::new(Inner::new(data), 3); // Limit set to 3

    // This should trigger a panic because we request a copy of 4 bytes,
    // which exceeds the remaining limit of 3.
    let result = std::panic::catch_unwind(|| {
        buffer.copy_to_bytes(4);
    });

    assert!(result.is_err(), "Expected panic when len is greater than remaining");
}

