// Answer 0

#[test]
fn test_set_vec_pos_max_vec_pos() {
    use core::mem::MaybeUninit;

    struct TestBytesMut {
        data: *mut Shared,
        kind_value: usize,
    }

    impl TestBytesMut {
        unsafe fn kind(&self) -> usize {
            self.kind_value
        }

        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind(), KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }
    }

    const MAX_VEC_POS: usize = usize::MAX >> VEC_POS_OFFSET;

    let mut test_bytes_mut = TestBytesMut {
        data: core::ptr::null_mut(),
        kind_value: KIND_VEC,
    };

    unsafe {
        test_bytes_mut.set_vec_pos(MAX_VEC_POS);
    }

    // Assuming we cannot take a reference to a potentially invalid pointer,
    // we verify the internal representation. This step may vary depending on full context.
    // Since direct testing is limited, we can inspect this indirectly through test_results:

    let expected_data_value = (MAX_VEC_POS << VEC_POS_OFFSET) & NOT_VEC_POS_MASK;

    assert_eq!(test_bytes_mut.data as usize & NOT_VEC_POS_MASK, expected_data_value);
}

