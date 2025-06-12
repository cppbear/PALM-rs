// Answer 0

#[test]
fn test_as_ptr_non_zero_sized() {
    struct NonZeroSized {
        value: i32,
    }

    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let value = NonZeroSized { value: 42 };
    let bucket = Bucket {
        ptr: NonNull::new(&value as *const _ as *mut _).unwrap(),
    };

    let raw_ptr = bucket.as_ptr();
    assert_eq!(raw_ptr, (&value as *const _ as *mut _) as *mut NonZeroSized);
}

#[test]
fn test_as_ptr_zero_sized() {
    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let bucket = Bucket {
        ptr: NonNull::new(&ZeroSized as *const _ as *mut _).unwrap(),
    };

    let raw_ptr = bucket.as_ptr();
    assert!(raw_ptr.is_null() || raw_ptr as usize % mem::align_of::<ZeroSized>() == 0);
}

