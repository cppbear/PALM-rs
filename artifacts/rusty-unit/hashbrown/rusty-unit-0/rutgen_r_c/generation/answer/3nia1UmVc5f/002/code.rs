// Answer 0

#[test]
fn test_from_base_index_non_zero_sized() {
    use std::ptr::NonNull;

    struct NonZeroSize;

    impl NonZeroSize {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSize))).unwrap();
    let index = 1; // A valid index for a non-zero-sized type

    unsafe {
        let bucket = Bucket::from_base_index(base, index);
        assert!(!bucket.ptr.as_ptr().is_null()); // Ensure pointer is not null
        // Validate that the pointer points to the expected location,
        // which should be base pointer subtracted by index.
        assert_eq!(bucket.ptr.as_ptr(), base.as_ptr().sub(index));
    }

    // Clean up the allocated memory
    unsafe { Box::from_raw(base.as_ptr()) };
}

#[test]
#[should_panic]
fn test_from_base_index_panics_out_of_bounds() {
    use std::ptr::NonNull;

    struct NonZeroSize;

    impl NonZeroSize {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSize))).unwrap();
    let index = 2; // Assume this index is out of bounds based on bucket mask.

    unsafe {
        Bucket::from_base_index(base, index);
    }

    // Clean up the allocated memory, which won't be reached due to panic
    unsafe { Box::from_raw(base.as_ptr()) };
}

