// Answer 0

#[test]
fn test_static_to_vec() {
    use std::ptr::null;
    use std::sync::atomic::AtomicPtr;

    let len = 5;
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let ptr: *const u8 = data.as_ptr();

    // Create an AtomicPtr pointing to null since we are not using it in this case
    let atomic_ptr = AtomicPtr::new(null::<()>());

    unsafe {
        let result = static_to_vec(&atomic_ptr, ptr, len);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_static_to_vec_zero_length() {
    use std::ptr::null;
    use std::sync::atomic::AtomicPtr;

    let len = 0;
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let ptr: *const u8 = data.as_ptr();

    let atomic_ptr = AtomicPtr::new(null::<()>());

    unsafe {
        let result = static_to_vec(&atomic_ptr, ptr, len);
        assert_eq!(result, Vec::<u8>::new());
    }
}

#[test]
#[should_panic]
fn test_static_to_vec_out_of_bounds() {
    use std::sync::atomic::AtomicPtr;

    let len = 6; // Length exceeds the actual data length
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let ptr: *const u8 = data.as_ptr();

    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let _ = static_to_vec(&atomic_ptr, ptr, len); // This should panic due to out-of-bounds access
    }
}

