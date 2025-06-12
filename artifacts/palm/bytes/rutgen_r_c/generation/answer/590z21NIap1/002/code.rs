// Answer 0

#[test]
fn test_advance_unchecked_count_zero() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
    assert_eq!(bytes_mut.len(), 10);
    assert_eq!(bytes_mut.capacity(), 10);
}

#[test]
fn test_advance_unchecked_count_equal_cap() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.advance_unchecked(10);
    }
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.capacity(), 0);
}

#[test]
fn test_advance_unchecked_with_max_vec_pos() {
    struct VecWrapper {
        bytes_mut: BytesMut,
    }

    impl VecWrapper {
        fn new() -> VecWrapper {
            let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
            unsafe { bytes_mut.set_vec_pos(2) }; // Set initial position

            VecWrapper { bytes_mut }
        }

        fn set_cap_and_advance(&mut self, count: usize) {
            // Assuming the capacity is at least 10 for this example
            unsafe {
                self.bytes_mut.advance_unchecked(count);
            }
        }
    }

    let mut wrapper = VecWrapper::new();
    let original_capacity = wrapper.bytes_mut.capacity();
    let max_vec_pos = usize::MAX >> 5; // Assuming the original provided shift
    wrapper.set_cap_and_advance(max_vec_pos - 2); // Advancing to exactly MAX_VEC_POS

    assert_eq!(wrapper.bytes_mut.len(), 0);
    assert_eq!(wrapper.bytes_mut.capacity(), original_capacity - (max_vec_pos - 2));
}

