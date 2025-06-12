// Answer 0

#[test]
fn test_inc_start_boundary_condition() {
    struct ByteBuffer {
        len: usize,
        ptr: *mut u8,
    }

    impl ByteBuffer {
        fn new(len: usize) -> Self {
            let layout = std::alloc::Layout::from_size_align(len, 1).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            ByteBuffer { len, ptr }
        }

        unsafe fn inc_start(&mut self, by: usize) {
            debug_assert!(self.len >= by, "internal: inc_start out of bounds");
            self.len -= by;
            self.ptr = self.ptr.add(by);
        }
    }

    let by = 1;
    let mut buffer = ByteBuffer::new(by);
    // Initial conditions
    assert_eq!(buffer.len, by);
    
    unsafe {
        buffer.inc_start(by);
    }

    // After inc_start, len should be 0
    assert_eq!(buffer.len, 0);
    assert_eq!(buffer.ptr, (buffer.ptr as usize + by) as *mut u8);
    
    // Cleanup
    unsafe { std::alloc::dealloc(buffer.ptr.sub(by), std::alloc::Layout::from_size_align(by, 1).unwrap()) };
}

