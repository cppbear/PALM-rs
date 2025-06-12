// Answer 0

#[test]
fn test_to_base_index_non_zero_sized() {
    use core::ptr::NonNull;

    struct NonZeroSized;

    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr = NonNull::new_unchecked(&NonZeroSized as *const _ as *mut NonZeroSized);
    let bucket = Bucket::<NonZeroSized> {
        ptr: NonNull::new_unchecked((base_ptr.as_ptr() as usize - 4 * core::mem::size_of::<NonZeroSized>()) as *mut NonZeroSized),
    };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
        assert_eq!(index, 4);
    }
}

#[test]
fn test_to_base_index_non_zero_sized_with_edge_case() {
    use core::ptr::NonNull;

    struct NonZeroSized;
    
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr = NonNull::new_unchecked(&NonZeroSized as *const _ as *mut NonZeroSized);
    let bucket = Bucket::<NonZeroSized> {
        ptr: NonNull::new_unchecked((base_ptr.as_ptr() as usize - 1 * core::mem::size_of::<NonZeroSized>()) as *mut NonZeroSized),
    };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
        assert_eq!(index, 1);
    }
}

#[test]
fn test_to_base_index_zero_sized() {
    use core::ptr::NonNull;

    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let base_ptr = NonNull::new_unchecked(&ZeroSized as *const _ as *mut ZeroSized);
    let bucket = Bucket::<ZeroSized> {
        ptr: NonNull::new_unchecked(base_ptr.as_ptr()),
    };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
        assert_eq!(index, 0);
    }
}

