// Answer 0

#[test]
fn test_as_mut_with_valid_pointer() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let mut test_instance = TestStruct { value: 42 };
    let non_null_ptr = NonNull::new(&mut test_instance.value).unwrap();

    let result: &mut i32;

    unsafe {
        // Cast the raw pointer back to a mutable reference
        result = non_null_ptr.as_mut();
    }

    *result = 100;

    assert_eq!(test_instance.value, 100);
}

#[test]
#[should_panic]
fn test_as_mut_with_invalid_pointer() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let non_null_ptr: NonNull<i32> = NonNull::new(std::ptr::null_mut()).unwrap_err(); // Attempt to create a NonNull from null-pointer which will panic

    unsafe {
        // This should trigger a panic due to originating from a null pointer
        let _result: &mut i32 = non_null_ptr.as_mut();
    }
}

#[test]
fn test_as_mut_with_reuse_pointer() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let mut test_instance = TestStruct { value: 0 };
    let non_null_ptr = NonNull::new(&mut test_instance.value).unwrap();

    unsafe {
        // First mutation
        let result1: &mut i32 = non_null_ptr.as_mut();
        *result1 = 500;

        // Second mutation, still valid as it's the same instance
        let result2: &mut i32 = non_null_ptr.as_mut();
        *result2 = 900;
    }

    assert_eq!(test_instance.value, 900);
}

