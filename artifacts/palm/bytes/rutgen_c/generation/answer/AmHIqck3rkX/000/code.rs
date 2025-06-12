// Answer 0

#[test]
fn test_owned_to_mut_valid_input() {
    use core::ptr::null;
    use alloc::boxed::Box;

    let data = Box::into_raw(Box::new(0u8));
    let length = 5;
    let slice = [1u8, 2, 3, 4, 5];
    let ptr = slice.as_ptr();

    unsafe {
        let bytes_mut = owned_to_mut(&AtomicPtr::new(data), ptr, length);
        assert_eq!(bytes_mut.len(), length);
    }
}

#[test]
#[should_panic]
fn test_owned_to_mut_invalid_length() {
    use core::ptr::null;
    use alloc::boxed::Box;

    let data = Box::into_raw(Box::new(0u8));
    let length = 10; // Length greater than the slice length (5)
    let slice = [1u8, 2, 3, 4, 5];
    let ptr = slice.as_ptr();

    unsafe {
        owned_to_mut(&AtomicPtr::new(data), ptr, length); // Should panic
    }
}

#[test]
fn test_owned_to_mut_zero_length() {
    use core::ptr::null;
    use alloc::boxed::Box;

    let data = Box::into_raw(Box::new(0u8));
    let length = 0;
    let slice: &[u8] = &[];

    unsafe {
        let bytes_mut = owned_to_mut(&AtomicPtr::new(data), slice.as_ptr(), length);
        assert_eq!(bytes_mut.len(), length);
    }
}

