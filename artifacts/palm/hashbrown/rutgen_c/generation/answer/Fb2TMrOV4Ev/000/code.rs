// Answer 0

#[test]
fn test_next_n_non_zero_sized() {
    struct TestType {
        value: i32,
    }
    
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_ptr: *mut TestType = Box::into_raw(Box::new(TestType { value: 0 }));
    let base_non_null = NonNull::new(base_ptr).unwrap();
    
    let bucket = Bucket { ptr: base_non_null };
    
    unsafe {
        let offset = 1;
        let new_bucket = bucket.next_n(offset);
        assert_eq!(new_bucket.ptr.as_ptr(), (base_ptr as usize - (offset * mem::size_of::<TestType>())) as *mut TestType);
        // Clean up
        drop(Box::from_raw(base_ptr));
    }
}

#[test]
fn test_next_n_zero_sized() {
    struct ZeroSizedType;

    impl ZeroSizedType {
        const IS_ZERO_SIZED: bool = true;
    }
    
    let base_ptr: *mut ZeroSizedType = Box::into_raw(Box::new(ZeroSizedType));
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: base_non_null };

    unsafe {
        let offset = 1;
        let new_bucket = bucket.next_n(offset);
        assert_eq!(new_bucket.ptr.as_ptr(), (base_ptr as usize + offset) as *mut ZeroSizedType);
        // Clean up
        drop(Box::from_raw(base_ptr));
    }
}

#[should_panic]
#[test]
fn test_next_n_out_of_bounds() {
    struct TestType {
        value: i32,
    }

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr: *mut TestType = Box::into_raw(Box::new(TestType { value: 0 }));
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: base_non_null };

    unsafe {
        // Here we will call next_n with an out of bound offset
        let _new_bucket = bucket.next_n(usize::MAX);
        // Clean up
        drop(Box::from_raw(base_ptr));
    }
}

