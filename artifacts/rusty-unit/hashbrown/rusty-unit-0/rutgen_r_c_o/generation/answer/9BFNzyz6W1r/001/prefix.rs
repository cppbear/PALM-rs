// Answer 0

#[test]
fn test_to_base_index_zero_sized() {
    struct ZeroSized;
    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let value1 = ZeroSized;
    let value2 = ZeroSized;

    let base_ptr = NonNull::from(&value1);
    let self_ptr = NonNull::from(&value2);

    let bucket = Bucket { ptr: self_ptr };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
    }
}

#[test]
fn test_to_base_index_zero_sized_different_positions() {
    struct ZeroSized;
    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let value1 = ZeroSized;
    let value2 = ZeroSized;

    let base_ptr = NonNull::from(&value2);
    let self_ptr = NonNull::from(&value1);

    let bucket = Bucket { ptr: self_ptr };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
    }
}

#[test]
fn test_to_base_index_zero_sized_with_offset() {
    struct ZeroSized;
    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let value1 = ZeroSized;
    let value2 = ZeroSized;

    let base_ptr = NonNull::from(&value1 as *const _ as *mut _);
    let self_ptr = NonNull::from(&value2 as *const _ as *mut _);

    let bucket = Bucket { ptr: self_ptr };

    unsafe {
        let index = bucket.to_base_index(base_ptr);
    }
}

