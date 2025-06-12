// Answer 0

#[test]
fn test_as_ptr_with_non_zero_sized_type() {
    struct TestType {
        value: i32,
    }

    let data = Box::into_raw(Box::new(TestType { value: 42 }));
    let bucket = Bucket {
        ptr: NonNull::new(data).unwrap(),
    };

    let ptr = bucket.as_ptr();
    assert_eq!(ptr, data.sub(1));
}

#[test]
fn test_as_ptr_with_zero_sized_type() {
    struct ZeroSized;

    let data = Box::into_raw(Box::new(ZeroSized));
    let bucket = Bucket {
        ptr: NonNull::new(data).unwrap(),
    };

    let ptr = bucket.as_ptr();
    assert_eq!(ptr as usize % mem::align_of::<ZeroSized>(), 0);
}

