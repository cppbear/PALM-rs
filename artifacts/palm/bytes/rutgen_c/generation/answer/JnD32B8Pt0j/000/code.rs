// Answer 0

#[test]
fn test_set_vec_pos_valid() {
    struct TestBytesMut {
        data: *mut Shared,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind, KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }

        fn kind(&self) -> usize {
            self.kind
        }
    }

    unsafe {
        let mut test_bytes_mut = TestBytesMut {
            data: core::ptr::null_mut(),
            kind: KIND_VEC,
        };

        test_bytes_mut.set_vec_pos(10);
        // Check if data was set correctly. In a real scenario, we would need a way to verify this.
    }
}

#[should_panic]
#[test]
fn test_set_vec_pos_invalid_kind() {
    struct TestBytesMut {
        data: *mut Shared,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind, KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }

        fn kind(&self) -> usize {
            self.kind
        }
    }

    unsafe {
        let mut test_bytes_mut = TestBytesMut {
            data: core::ptr::null_mut(),
            kind: KIND_ARC, // Invalid kind
        };

        test_bytes_mut.set_vec_pos(10); // This should panic due to invalid kind.
    }
}

#[should_panic]
#[test]
fn test_set_vec_pos_out_of_bounds() {
    struct TestBytesMut {
        data: *mut Shared,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind, KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }

        fn kind(&self) -> usize {
            self.kind
        }
    }

    unsafe {
        let mut test_bytes_mut = TestBytesMut {
            data: core::ptr::null_mut(),
            kind: KIND_VEC,
        };

        test_bytes_mut.set_vec_pos(MAX_VEC_POS + 1); // This should panic due to out of bounds.
    }
}

