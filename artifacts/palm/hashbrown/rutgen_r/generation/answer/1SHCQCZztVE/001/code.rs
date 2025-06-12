// Answer 0

#[test]
fn test_as_mut_valid_reference() {
    struct TestStruct {
        value: i32,
    }

    let mut test_instance = TestStruct { value: 42 };
    let ptr = &mut test_instance.value as *mut i32;

    unsafe {
        let mut_ref: &mut i32 = std::ptr::NonNull::new_unchecked(ptr).as_mut();
        *mut_ref += 1;
        assert_eq!(*mut_ref, 43);
    }
}

#[test]
#[should_panic]
fn test_as_mut_invalid_reference() {
    struct BadStruct {
        value: String,
    }

    let mut bad_instance = BadStruct { value: "test".to_string() };
    let ptr = &mut bad_instance.value as *mut String;

    // Here we will create a dangling reference by moving out the value
    let _temp = std::mem::take(&mut bad_instance.value);

    unsafe {
        // This should panic because the reference is not valid anymore
        let _mut_ref: &mut String = std::ptr::NonNull::new_unchecked(ptr).as_mut();
    }
}

#[test]
fn test_as_mut_multiple_accesses() {
    struct MultiAccessStruct {
        value: f64,
    }

    let mut multi_instance = MultiAccessStruct { value: 3.14 };
    let ptr = &mut multi_instance.value as *mut f64;

    unsafe {
        let mut_ref1: &mut f64 = std::ptr::NonNull::new_unchecked(ptr).as_mut();
        *mut_ref1 += 0.86;

        let mut_ref2: &mut f64 = std::ptr::NonNull::new_unchecked(ptr).as_mut();
        *mut_ref2 *= 2.0;

        assert_eq!(*mut_ref1, 4.0);
        assert_eq!(*mut_ref2, 8.0);
    }
}

