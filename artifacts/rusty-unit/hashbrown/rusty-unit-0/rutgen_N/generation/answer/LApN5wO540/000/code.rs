// Answer 0

#[test]
fn test_as_non_null_valid() {
    use std::ptr::NonNull;

    struct Data {
        ptr: *mut i32,
    }

    let value = Box::into_raw(Box::new(5));
    let data = Data { ptr: value };

    let non_null_ptr: NonNull<i32> = unsafe { NonNull::new_unchecked(data.ptr) };
    assert_eq!(non_null_ptr.as_ref(), &5);
}

#[test]
#[should_panic]
fn test_as_non_null_null_pointer() {
    use std::ptr::NonNull;

    struct Data {
        ptr: *mut i32,
    }

    let data = Data { ptr: std::ptr::null_mut() };

    // This will panic because we are calling a method that expects a non-null pointer
    let _non_null_ptr: NonNull<i32> = unsafe { NonNull::new_unchecked(data.ptr) };
}

