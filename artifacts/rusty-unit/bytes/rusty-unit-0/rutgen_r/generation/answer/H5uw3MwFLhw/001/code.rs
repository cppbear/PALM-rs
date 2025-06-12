// Answer 0

#[test]
fn test_from_raw_parts_mut_valid() {
    let mut bytes = vec![0u8; 10];
    let ptr = bytes.as_mut_ptr();
    let len = bytes.len();

    unsafe {
        let slice = UninitSlice::from_raw_parts_mut(ptr, len);
        assert_eq!(slice.len(), 10);
    }
}

#[test]
#[should_panic]
fn test_from_raw_parts_mut_zero_length() {
    let mut bytes = vec![0u8; 10];
    let ptr = bytes.as_mut_ptr();
    
    unsafe {
        let slice = UninitSlice::from_raw_parts_mut(ptr, 0);
        // Accessing the slice after creating it should panic
        let _ = slice.len();
    }
}

#[test]
#[should_panic]
fn test_from_raw_parts_mut_invalid_ptr() {
    let ptr: *mut u8 = core::ptr::null_mut();
    let len = 10;

    unsafe {
        // Using a null pointer should cause a panic
        let _ = UninitSlice::from_raw_parts_mut(ptr, len);
    }
}

#[test]
fn test_from_raw_parts_mut_partial_init() {
    let mut bytes = vec![0u8; 10];
    let ptr = bytes.as_mut_ptr();
    let len = 5; // len less than the actual allocated slice

    unsafe {
        let slice = UninitSlice::from_raw_parts_mut(ptr, len);
        assert_eq!(slice.len(), 5);
    }
}

