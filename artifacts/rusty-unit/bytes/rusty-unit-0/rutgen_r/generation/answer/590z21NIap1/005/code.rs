// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }

    impl BytesMut {
        fn new(cap: usize) -> Self {
            let buffer = vec![0u8; cap];
            let ptr = buffer.as_ptr() as *mut u8;
            std::mem::forget(buffer); // Prevent the buffer from being dropped
            BytesMut { ptr, len: cap, cap }
        }

        unsafe fn advance_unchecked(&mut self, count: usize) {
            if count == 0 {
                return;
            }

            debug_assert!(count <= self.cap, "internal: set_start out of bounds");
            // Simulating some behavior as the original code is not provided
            self.ptr = self.ptr.add(count);
            self.len = self.len.checked_sub(count).unwrap_or(0);
            self.cap -= count;
        }
    }

    let mut bytes_mut = BytesMut::new(10);
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
    assert_eq!(bytes_mut.len, 10);
    assert_eq!(bytes_mut.cap, 10);
}

#[test]
#[should_panic]
fn test_advance_unchecked_exceeds_capacity() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }

    impl BytesMut {
        fn new(cap: usize) -> Self {
            let buffer = vec![0u8; cap];
            let ptr = buffer.as_ptr() as *mut u8;
            std::mem::forget(buffer); // Prevent the buffer from being dropped
            BytesMut { ptr, len: cap, cap }
        }

        unsafe fn advance_unchecked(&mut self, count: usize) {
            if count == 0 {
                return;
            }

            debug_assert!(count <= self.cap, "internal: set_start out of bounds");
            // Simulating some behavior
            self.ptr = self.ptr.add(count);
            self.len = self.len.checked_sub(count).unwrap_or(0);
            self.cap -= count;
        }
    }

    let mut bytes_mut = BytesMut::new(10);
    unsafe {
        bytes_mut.advance_unchecked(15); // This should trigger a panic
    }
}

