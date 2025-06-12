// Answer 0

#[test]
fn test_bucket_drop_zero_sized() {
    struct ZeroSizedStruct;
    impl ZeroSizedStruct {
        const IS_ZERO_SIZED: bool = true;
    }
    
    let base = NonNull::new(Box::into_raw(Box::new(ZeroSizedStruct))).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base, 0) };
    unsafe {
        bucket.drop();
    }
}

#[test]
fn test_bucket_drop_non_zero_sized() {
    struct NonZeroSizedStruct {
        data: u32,
    }
    impl NonZeroSizedStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSizedStruct { data: 10 }))).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base, 0) };
    unsafe {
        bucket.drop();
    }
}

#[test]
#[should_panic]
fn test_bucket_drop_out_of_bounds_index() {
    struct NonZeroSizedStruct {
        data: u32,
    }
    impl NonZeroSizedStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSizedStruct { data: 10 }))).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base, 1) };
    unsafe {
        bucket.drop();
    }
}

#[test]
fn test_bucket_drop_with_offset() {
    struct NonZeroSizedStruct {
        data: u32,
    }
    impl NonZeroSizedStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSizedStruct { data: 10 }))).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base, 0) };
    // Testing drop when no offset applied
    unsafe {
        bucket.next_n(0).drop();
    }
}

#[test]
fn test_bucket_drop_multiple() {
    struct NonZeroSizedStruct {
        data: u32,
    }
    impl NonZeroSizedStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base1 = NonNull::new(Box::into_raw(Box::new(NonZeroSizedStruct { data: 10 }))).unwrap();
    let base2 = NonNull::new(Box::into_raw(Box::new(NonZeroSizedStruct { data: 20 }))).unwrap();

    let bucket1 = unsafe { Bucket::from_base_index(base1, 0) };
    let bucket2 = unsafe { Bucket::from_base_index(base2, 0) };

    unsafe {
        bucket1.drop();
        bucket2.drop();
    }
}

