// Answer 0

#[test]
fn test_chunk_mut_grows_buffer() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new() -> Self {
            TestBuf { data: Vec::new() }
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.data.as_mut_ptr()
        }

        fn reserve(&mut self, additional: usize) {
            self.data.reserve(additional);
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            if self.capacity() == self.len() {
                self.reserve(64);
            }

            let cap = self.capacity();
            let len = self.len();

            let ptr = self.as_mut_ptr();
            unsafe { UninitSlice::from_raw_parts_mut(ptr.add(len), cap - len) }
        }
    }

    struct UninitSlice {
        ptr: *mut u8,
        len: usize,
    }

    impl UninitSlice {
        unsafe fn from_raw_parts_mut(ptr: *mut u8, len: usize) -> &mut Self {
            &mut *(ptr as *mut UninitSlice)
        }
    }

    let mut buf = TestBuf::new();
    let slice = buf.chunk_mut();

    assert!(slice.len > 0); // Ensure the uninitialized slice has non-zero length
}

#[test]
fn test_chunk_mut_no_growth() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf { data: Vec::with_capacity(size) }
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.data.as_mut_ptr()
        }

        fn reserve(&mut self, _: usize) {
            // No-op since we are testing no growth
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            if self.capacity() == self.len() {
                self.reserve(64);
            }

            let cap = self.capacity();
            let len = self.len();

            let ptr = self.as_mut_ptr();
            unsafe { UninitSlice::from_raw_parts_mut(ptr.add(len), cap - len) }
        }
    }

    struct UninitSlice {
        ptr: *mut u8,
        len: usize,
    }

    impl UninitSlice {
        unsafe fn from_raw_parts_mut(ptr: *mut u8, len: usize) -> &mut Self {
            &mut *(ptr as *mut UninitSlice)
        }
    }

    let mut buf = TestBuf::new(10);
    let slice = buf.chunk_mut();

    assert!(slice.len > 0); // Ensure the uninitialized slice has non-zero length
}

