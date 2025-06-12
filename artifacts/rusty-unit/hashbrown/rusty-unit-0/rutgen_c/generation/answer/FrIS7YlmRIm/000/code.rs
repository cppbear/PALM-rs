// Answer 0

#[cfg(test)]
fn test_next_impl() {
    struct TestStruct;

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestStruct as *mut _),
    };

    let ctrl: *const u8 = &data as *const _ as *const u8;
    let len = 1;

    unsafe {
        let mut iter_range = RawIterRange::new(ctrl, data, len);
        let bucket = iter_range.next_impl::<false>();

        assert!(bucket.is_some());
    }
}

#[cfg(test)]
fn test_next_impl_no_check() {
    struct TestStruct;

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestStruct as *mut _),
    };

    let ctrl: *const u8 = &data as *const _ as *const u8;
    let len = 1;

    unsafe {
        let mut iter_range = RawIterRange::new(ctrl, data, len);
        let bucket = iter_range.next_impl::<false>();

        assert!(bucket.is_some());
        
        let _bucket_empty = iter_range.next_impl::<false>();
        assert!(iter_range.next_ctrl <= iter_range.end);
    }
}

#[cfg(test)]
fn test_next_impl_with_check() {
    struct TestStruct;

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestStruct as *mut _),
    };

    let ctrl: *const u8 = &data as *const _ as *const u8;
    let len = 1;

    unsafe {
        let mut iter_range = RawIterRange::new(ctrl, data, len);
        let bucket = iter_range.next_impl::<true>();

        assert!(bucket.is_some());

        let _bucket_empty = iter_range.next_impl::<true>();
        assert!(iter_range.next_ctrl >= iter_range.end);
    }
}

