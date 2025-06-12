// Answer 0

#[test]
fn test_chunk_mut_grows_and_returns_uninit_slice() {
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

        fn reserve(&mut self, additional: usize) {
            self.data.reserve(additional);
        }

        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.data.as_mut_ptr()
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            if self.capacity() == self.len() {
                self.reserve(64); // Grow the vec
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
    buf.data.resize(16, 0); // Set len to 16 and capacity will be 16 by default

    // Initial capacity and length are both 16, so calling chunk_mut should trigger reserve
    let slice = buf.chunk_mut();
    
    // Check that the slice points to the right memory and has the correct length
    assert_eq!(slice.len, 48); // Capacity after reserve should now be 64, so remaining length is 64 - 16 = 48
}

