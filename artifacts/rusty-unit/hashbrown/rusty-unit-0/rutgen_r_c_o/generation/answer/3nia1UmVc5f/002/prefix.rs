// Answer 0

#[test]
fn test_from_base_index_valid_non_zero_sized() {
    struct NonZeroSized;
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base = NonNull::new(0x1000 as *mut NonZeroSized).unwrap();
    let index = 1; // Valid index within the range

    unsafe {
        from_base_index(base, index);
    }
}

#[test]
fn test_from_base_index_boundary_condition() {
    struct NonZeroSized;
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(0x1000 as *mut NonZeroSized).unwrap();
    let index = RawTableInner.bucket_mask; // Valid index at the upper boundary

    unsafe {
        from_base_index(base, index);
    }
}

#[test]
fn test_from_base_index_middle_value() {
    struct NonZeroSized;
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(0x1000 as *mut NonZeroSized).unwrap();
    let index = 5; // Valid index in the middle of the range

    unsafe {
        from_base_index(base, index);
    }
}

#[test]
fn test_from_base_index_invalid_too_high() {
    struct NonZeroSized;
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(0x1000 as *mut NonZeroSized).unwrap();
    let index = RawTableInner.bucket_mask + 1; // Invalid index exceeding the limit

    #[should_panic]
    unsafe {
        from_base_index(base, index);
    }
}

#[test]
fn test_from_base_index_zero_index() {
    struct NonZeroSized;
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(0x1000 as *mut NonZeroSized).unwrap();
    let index = 0; // Valid index at the lower boundary

    unsafe {
        from_base_index(base, index);
    }
}

