// Answer 0

#[test]
fn test_get_vec_pos() {
    struct TestBytesMut {
        data: *mut u8,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn get_vec_pos(&self) -> usize {
            debug_assert_eq!(self.kind, KIND_VEC);
            self.data as usize >> VEC_POS_OFFSET
        }
    }

    let mut shared_data = vec![0u8; 32].into_boxed_slice();
    let data_ptr = shared_data.as_mut_ptr();
    let kind_vec = KIND_VEC;

    let test_bytes_mut = TestBytesMut {
        data: data_ptr,
        kind: kind_vec,
    };

    let expected_vec_pos = data_ptr as usize >> VEC_POS_OFFSET;
    let actual_vec_pos = unsafe { test_bytes_mut.get_vec_pos() };

    assert_eq!(actual_vec_pos, expected_vec_pos);
}

#[test]
#[should_panic]
fn test_get_vec_pos_should_panic_not_kind_vec() {
    struct TestBytesMut {
        data: *mut u8,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn get_vec_pos(&self) -> usize {
            debug_assert_eq!(self.kind, KIND_VEC);
            self.data as usize >> VEC_POS_OFFSET
        }
    }

    let mut shared_data = vec![0u8; 32].into_boxed_slice();
    let data_ptr = shared_data.as_mut_ptr();
    let kind_arc = KIND_ARC; // Setting incorrect kind to trigger panic

    let test_bytes_mut = TestBytesMut {
        data: data_ptr,
        kind: kind_arc,
    };

    unsafe { test_bytes_mut.get_vec_pos() }; // This should panic due to kind assertion.
}

#[test]
fn test_get_vec_pos_with_zero_data() {
    struct TestBytesMut {
        data: *mut u8,
        kind: usize,
    }

    impl TestBytesMut {
        unsafe fn get_vec_pos(&self) -> usize {
            debug_assert_eq!(self.kind, KIND_VEC);
            self.data as usize >> VEC_POS_OFFSET
        }
    }

    let data_ptr: *mut u8 = ptr::null_mut();
    let kind_vec = KIND_VEC;

    let test_bytes_mut = TestBytesMut {
        data: data_ptr,
        kind: kind_vec,
    };

    let expected_vec_pos = data_ptr as usize >> VEC_POS_OFFSET; // expected to be zero
    let actual_vec_pos = unsafe { test_bytes_mut.get_vec_pos() };

    assert_eq!(actual_vec_pos, expected_vec_pos);
}

