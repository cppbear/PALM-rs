// Answer 0

#[test]
fn test_from_base_index_zero_sized() {
    use std::ptr::NonNull;

    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new(Box::into_raw(Box::new(ZeroSized)) as *mut ZeroSized).unwrap();
    let index: usize = 0; // Lower bound valid index

    unsafe {
        let result = from_base_index::<ZeroSized>(base, index);
        assert!(!result.ptr.as_ptr().is_null());
    }

    let base = NonNull::new(Box::into_raw(Box::new(ZeroSized)) as *mut ZeroSized).unwrap();
    let index: usize = 1; // Example of valid upper bound index

    unsafe {
        let result = from_base_index::<ZeroSized>(base, index);
        assert!(!result.ptr.as_ptr().is_null());
    }

    // Clean up memory
    unsafe {
        let _ = Box::from_raw(base.as_ptr());
    }
}

#[test]
#[should_panic]
fn test_from_base_index_out_of_bounds() {
    use std::ptr::NonNull;

    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new(Box::into_raw(Box::new(ZeroSized)) as *mut ZeroSized).unwrap();
    let index: usize = 2; // Assuming bucket_mask is 1, this should panic

    unsafe {
        from_base_index::<ZeroSized>(base, index);
    }
}

