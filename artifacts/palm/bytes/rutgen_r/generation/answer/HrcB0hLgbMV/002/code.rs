// Answer 0

#[test]
fn test_chunk_mut_with_available_capacity() {
    struct Buf {
        data: Vec<u8>,
    }

    impl Buf {
        fn new() -> Self {
            Buf { data: Vec::new() }
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

        fn chunk_mut(&mut self) -> &mut [MaybeUninit<u8>] {
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
        unsafe fn from_raw_parts_mut(ptr: *mut u8, len: usize) -> &mut [MaybeUninit<u8>] {
            std::slice::from_raw_parts_mut(ptr as *mut MaybeUninit<u8>, len)
        }
    }

    let mut buf = Buf::new();
    buf.data.resize(64, 0); // Pre-allocate enough capacity
    buf.data.truncate(32); // Set length to 32, which is less than capacity

    let slice = buf.chunk_mut();
    assert_eq!(slice.len(), buf.capacity() - buf.len());
}

