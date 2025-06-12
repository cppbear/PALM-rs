// Answer 0

#[test]
fn test_bucket_as_mut_non_zero_sized() {
    struct NonZeroSized {
        value: i32,
    }

    let mut value = NonZeroSized { value: 42 };
    let ptr = NonNull::new(&mut value as *mut _).unwrap();
    let bucket = Bucket { ptr };

    unsafe {
        let mut_ref = bucket.as_mut();
        assert_eq!(mut_ref.value, 42);
        mut_ref.value = 100;
        assert_eq!(value.value, 100);
    }
}

#[test]
fn test_bucket_as_mut_zero_sized() {
    struct ZeroSized;

    let value = ZeroSized;
    let ptr = NonNull::new(&value as *const _ as *mut _).unwrap();
    let bucket = Bucket { ptr };

    unsafe {
        let _mut_ref = bucket.as_mut(); // Safety is bypassed because ZeroSized does not cause UB
    }
}

