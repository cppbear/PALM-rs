// Answer 0

#[test]
fn test_write_non_zero_sized() {
    struct TestStruct {
        value: i32,
    }
    
    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let mut value = TestStruct { value: 42 };
    let base_index = NonNull::from(&mut value);
    let bucket = unsafe { Bucket::from_base_index(base_index, 0) };
    
    unsafe {
        bucket.write(TestStruct { value: 100 });
    }
}

#[test]
fn test_write_zero_sized() {
    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }
    
    let mut value = ZeroSized;
    let base_index = NonNull::from(&mut value);
    let bucket = unsafe { Bucket::from_base_index(base_index, 0) };
    
    unsafe {
        bucket.write(ZeroSized);
    }
}

#[test]
fn test_write_with_invalid_pointer() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let mut value = TestStruct { value: 42 };
    let null_pointer = NonNull::new(std::ptr::null_mut()).unwrap_err();
    let bucket = Bucket { ptr: null_pointer };
    
    unsafe {
        bucket.write(TestStruct { value: 100 });
    }
}

#[test]
#[should_panic]
fn test_write_to_null_pointer() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_index = NonNull::from(&mut 5);
    let bucket = unsafe { Bucket::from_base_index(base_index, usize::MAX) };
    
    unsafe {
        bucket.write(TestStruct { value: 100 });
    }
}

