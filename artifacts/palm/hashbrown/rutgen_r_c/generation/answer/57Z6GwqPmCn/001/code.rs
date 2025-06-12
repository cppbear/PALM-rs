// Answer 0

#[test]
fn test_write_to_non_zero_sized() {
    struct TestStruct {
        value: u32,
    }

    let mut base: Box<TestStruct> = Box::new(TestStruct { value: 42 });
    let base_ptr = NonNull::new(Box::into_raw(base)).unwrap();
    let bucket = Bucket {
        ptr: base_ptr,
    };

    unsafe {
        bucket.write(TestStruct { value: 99 });
        let updated = &*bucket.as_ptr();
        assert_eq!(updated.value, 99);
    }
}

#[test]
fn test_write_to_zero_sized() {
    #[derive(Debug)]
    struct ZeroSized;

    let base: Box<ZeroSized> = Box::new(ZeroSized);
    let base_ptr = NonNull::new(Box::into_raw(base)).unwrap();
    let bucket = Bucket {
        ptr: base_ptr,
    };

    unsafe {
        bucket.write(ZeroSized);
        // No way to effectively check for zero-sized types
    }
}

#[should_panic]
#[test]
fn test_write_invalid_pointer() {
    struct TestStruct {
        value: u32,
    }

    let invalid_base_ptr: NonNull<TestStruct> = NonNull::dangling();
    let bucket = Bucket {
        ptr: invalid_base_ptr,
    };

    unsafe {
        bucket.write(TestStruct { value: 42 });
    }
}

