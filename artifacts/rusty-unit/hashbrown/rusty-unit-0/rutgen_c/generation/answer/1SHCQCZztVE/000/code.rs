// Answer 0

#[test]
fn test_as_mut() {
    use core::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    let mut data = TestStruct { value: 42 };
    let non_null_ptr = NonNull::new(&mut data as *mut _).unwrap();
    let bucket = Bucket { ptr: non_null_ptr };

    unsafe {
        let mutable_ref = bucket.as_mut();
        mutable_ref.value = 100;
    }

    assert_eq!(data.value, 100);
}

#[test]
#[should_panic]
fn test_as_mut_invalid_pointer() {
    use core::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    // Simulate an invalid `NonNull` by creating an instance and moving it
    let data = TestStruct { value: 42 };
    let mut non_null_ptr = NonNull::new(&data as *const _ as *mut _).unwrap();
    let bucket = Bucket { ptr: non_null_ptr };

    unsafe {
        let _mutable_ref = bucket.as_mut(); // This should panic as the data is no longer valid
    }
}

