// Answer 0

#[test]
fn test_from_raw_parts_mut_valid_input() {
    let mut buffer = [0u8; 10];
    let ptr = buffer.as_mut_ptr();
    let len = buffer.len();

    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(ptr, len);
        assert_eq!(uninit_slice.len(), len);
    }
}

#[test]
#[should_panic]
fn test_from_raw_parts_mut_zero_length() {
    let mut buffer: &mut [u8] = &mut [];
    let ptr = buffer.as_mut_ptr();
    let len = 0;

    unsafe {
        UninitSlice::from_raw_parts_mut(ptr, len);
    }
}

#[test]
fn test_from_raw_parts_mut_non_zero_length() {
    let mut buffer = [1u8; 5];
    let ptr = buffer.as_mut_ptr();
    let len = buffer.len();

    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(ptr, len);
        for i in 0..len {
            assert_eq!(uninit_slice.as_uninit_slice_mut()[i].assume_init_or(0), 1);
        }
    }
}

#[test]
fn test_from_raw_parts_mut_invalid_pointer() {
    let len = 10;

    unsafe {
        let invalid_ptr: *mut u8 = core::ptr::null_mut();
        let result = UninitSlice::from_raw_parts_mut(invalid_ptr, len);
        assert!(result.len() == len);
    }
}

