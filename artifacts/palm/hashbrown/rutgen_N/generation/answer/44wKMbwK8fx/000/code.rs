// Answer 0

#[test]
fn test_as_ref_valid_pointer() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let value = TestStruct { value: 42 };
    let ptr = NonNull::new(&value as *const TestStruct as *mut TestStruct).unwrap();

    unsafe {
        let ref_value: &TestStruct = ptr.as_ref();
        assert_eq!(ref_value.value, 42);
    }
}

#[test]
#[should_panic]
fn test_as_ref_null_pointer() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let ptr: NonNull<TestStruct> = NonNull::new(std::ptr::null_mut()).unwrap_err();

    unsafe {
        // Trying to dereference a null pointer should panic.
        let _ref_value: &TestStruct = ptr.as_ref();
    }
}

