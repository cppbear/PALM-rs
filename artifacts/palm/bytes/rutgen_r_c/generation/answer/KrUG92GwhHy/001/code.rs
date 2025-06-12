// Answer 0

#[test]
fn test_write_str_success() {
    use std::fmt::Write;

    struct TestBufMut {
        data: BytesMut,
    }

    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.cap - self.data.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.len += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume implementation exists to return an UninitSlice
        }

        // Other trait methods would need to be implemented as well.
        fn put_slice(&mut self, src: &[u8]) {
            // Assume concrete method implementation to put slice in the buffer
        }
    }

    let mut buf_mut = TestBufMut {
        data: BytesMut {
            ptr: NonNull::new_unchecked(Box::into_raw(Box::new(0u8))),
            len: 10,
            cap: 10, // Set capacity to exactly match the string length
            data: ptr::null_mut(),
        },
    };

    let result = buf_mut.write_str("test_str");
    assert_eq!(result, Ok(()));
}

