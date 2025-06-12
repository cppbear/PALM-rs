// Answer 0

#[test]
fn test_write_str_success() {
    struct TestBufMut {
        bytes: BytesMut,
    }

    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.bytes.cap - self.bytes.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.bytes.len += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Simplified implementation for test purpose
            unsafe { &mut *(self.bytes.ptr.as_ptr() as *mut UninitSlice) }
        }

        // Other trait methods would be required here, but are omitted for brevity 
    }

    let mut buf_mut = TestBufMut {
        bytes: BytesMut {
            ptr: NonNull::new(Box::into_raw(Box::new([0u8; 10]))).unwrap(),
            len: 0,
            cap: 10,
            data: std::ptr::null_mut(),
        },
    };

    assert!(buf_mut.write_str("test").is_ok());
    assert_eq!(buf_mut.bytes.len, 4);
}

#[test]
fn test_write_str_failure() {
    struct TestBufMut {
        bytes: BytesMut,
    }

    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.bytes.cap - self.bytes.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.bytes.len += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Simplified implementation for test purpose
            unsafe { &mut *(self.bytes.ptr.as_ptr() as *mut UninitSlice) }
        }

        // Other trait methods would be required here, but are omitted for brevity 
    }

    let mut buf_mut = TestBufMut {
        bytes: BytesMut {
            ptr: NonNull::new(Box::into_raw(Box::new([0u8; 4]))).unwrap(),
            len: 0,
            cap: 4,
            data: std::ptr::null_mut(),
        },
    };

    assert!(buf_mut.write_str("longer string").is_err());
}

