// Answer 0

#[test]
fn test_next_n_zero_sized_type_zero_offset() {
    struct ZeroSizedType;
    impl ZeroSizedType {
        const IS_ZERO_SIZED: bool = true;
    }
    let base = NonNull::new_unchecked(&ZeroSizedType as *const _ as *mut _);
    let bucket = Bucket { ptr: base };
    unsafe {
        let _ = bucket.next_n(0);
    }
}

#[test]
fn test_next_n_zero_sized_type_valid_offset() {
    struct ZeroSizedType;
    impl ZeroSizedType {
        const IS_ZERO_SIZED: bool = true;
    }
    let base = NonNull::new_unchecked(&ZeroSizedType as *const _ as *mut _);
    let bucket = Bucket { ptr: base };
    
    unsafe {
        let _ = bucket.next_n(1);
    }
}

#[test]
fn test_next_n_zero_sized_type_max_offset() {
    struct ZeroSizedType;
    impl ZeroSizedType {
        const IS_ZERO_SIZED: bool = true;
    }
    let base = NonNull::new_unchecked(&ZeroSizedType as *const _ as *mut _);
    let bucket = Bucket { ptr: base };

    unsafe {
        let offset = RawTableInner.bucket_mask; // Assume this is defined somewhere
        let _ = bucket.next_n(offset);
    }
}

#[test]
#[should_panic]
fn test_next_n_zero_sized_type_invalid_offset() {
    struct ZeroSizedType;
    impl ZeroSizedType {
        const IS_ZERO_SIZED: bool = true;
    }
    let base = NonNull::new_unchecked(&ZeroSizedType as *const _ as *mut _);
    let bucket = Bucket { ptr: base };

    unsafe {
        let offset = RawTableInner.bucket_mask + 1; // This should panic
        let _ = bucket.next_n(offset);
    }
}

