// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    let mut buffer = Buffer::new(vec![1, 2, 3, 4], 4); // capacity is 4
    unsafe {
        buffer.advance_unchecked(0); // should not panic, count is 0
    }
    assert_eq!(buffer.len, 4); // length remains unchanged
    assert_eq!(buffer.cap, 4); // capacity remains unchanged
}

#[test]
fn test_advance_unchecked_full_capacity() {
    let mut buffer = Buffer::new(vec![1, 2, 3, 4], 4); // capacity is 4
    unsafe {
        buffer.advance_unchecked(4); // should not panic, count equals capacity
    }
    assert_eq!(buffer.len, 0); // length should be updated to 0
    assert_eq!(buffer.cap, 0); // capacity should be updated to 0
}

#[test]
fn test_advance_unchecked_non_vec_kind() {
    struct NonVecBuffer {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: usize,
    }

    impl NonVecBuffer {
        fn new(len: usize, cap: usize) -> Self {
            let vec = vec![0u8; cap];
            let ptr = vec.as_ptr() as *mut u8;
            std::mem::forget(vec);
            NonVecBuffer { ptr, len, cap, kind: 0 } // assume kind indicates non-vec
        }

        unsafe fn advance_unchecked(&mut self, count: usize) {
            if count == 0 {
                return;
            }

            if self.kind != 1 { // KIND_VEC = 1, so ensure kind indicates not vec
                return;
            }

            self.ptr = self.ptr.add(count);
            self.len = self.len.checked_sub(count).unwrap_or(0);
            self.cap -= count;
        }
    }
    
    let mut buffer = NonVecBuffer::new(5, 5); // capacity is 5
    unsafe {
        buffer.advance_unchecked(3); // should not panic
    }
    assert_eq!(buffer.len, 2); // length should be updated to 2
    assert_eq!(buffer.cap, 2); // capacity should be updated to 2
}

