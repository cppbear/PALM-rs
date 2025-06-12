// Answer 0

#[test]
fn test_write_str_insufficient_capacity() {
    struct TestBufMut {
        bytes_mut: BytesMut,
    }

    impl TestBufMut {
        fn new(len: usize, cap: usize) -> Self {
            let vec = vec![0u8; cap];
            let ptr = NonNull::from(vec.as_ptr() as *mut u8);
            // Safety: We're creating a valid BytesMut with enough capacity.
            let bytes_mut = BytesMut {
                ptr,
                len,
                cap,
                data: ptr.as_ptr() as *mut Shared,
            };
            Self { bytes_mut }
        }
    }

    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.bytes_mut.cap - self.bytes_mut.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.bytes_mut.len += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Not relevant for this test
            unimplemented!()
        }

        fn put<T: super::Buf>(&mut self, _src: T) {
            // Not relevant for this test
            unimplemented!()
        }

        // Additional methods can be omitted if not directly used.
    }

    let mut buffer = TestBufMut::new(5, 5); // Current length is less than capacity
    let result = buffer.bytes_mut.write_str("Hello, World!"); // Length of string exceeds remaining capacity

    assert!(result.is_err()); // Expected to return Err(fmt::Error)
}

