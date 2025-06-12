// Answer 0

#[test]
fn test_copy_to_bytes_under_limit() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { data, position: 0 }
        }
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let end = self.position + len;
            let bytes = self.data[self.position..end].to_vec();
            self.position += len;
            crate::Bytes {
                ptr: bytes.as_ptr(),
                len: bytes.len(),
                data: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
                vtable: std::ptr::null(),
            }
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let mut mock_buf = MockBuf::new(data);
    let limit = mock_buf.remaining(); // limit is set to remaining bytes
    let mut take = Take { inner: mock_buf, limit };

    let result = take.copy_to_bytes(limit);
    assert_eq!(result.len, limit);
}

#[test]
#[should_panic(expected = "`len` greater than remaining")]
fn test_copy_to_bytes_over_limit() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { data, position: 0 }
        }
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let end = self.position + len;
            let bytes = self.data[self.position..end].to_vec();
            self.position += len;
            crate::Bytes {
                ptr: bytes.as_ptr(),
                len: bytes.len(),
                data: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
                vtable: std::ptr::null(),
            }
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let data = vec![1, 2, 3];
    let mut mock_buf = MockBuf::new(data);
    let limit = mock_buf.remaining(); // limit is 3
    let mut take = Take { inner: mock_buf, limit };

    take.copy_to_bytes(limit + 1); // Trying to copy more than remaining
}

