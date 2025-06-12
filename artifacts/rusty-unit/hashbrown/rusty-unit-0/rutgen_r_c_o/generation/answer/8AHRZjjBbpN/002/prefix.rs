// Answer 0

#[test]
fn test_as_ptr_non_zero_sized() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base: NonNull<TestStruct> = NonNull::new(Box::into_raw(Box::new(TestStruct { value: 42 }))).unwrap();
    let index: usize = 1;

    let bucket = unsafe { Bucket::from_base_index(base, index) };
    let ptr = bucket.as_ptr();
}

#[test]
fn test_as_ptr_non_zero_sized_edge() {
    struct EdgeStruct {
        value: i32,
    }

    impl EdgeStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base: NonNull<EdgeStruct> = NonNull::new(Box::into_raw(Box::new(EdgeStruct { value: 100 }))).unwrap();
    let index: usize = 1; // Testing a valid index

    let bucket = unsafe { Bucket::from_base_index(base, index) };
    let ptr = bucket.as_ptr();
}

#[test]
#[should_panic] // This test will panic if index is invalid (i.e., out of range)
fn test_as_ptr_non_zero_sized_invalid_index() {
    struct InvalidIndexStruct {
        value: i32,
    }

    impl InvalidIndexStruct {
        const IS_ZERO_SIZED: bool = false;
    }

    let base: NonNull<InvalidIndexStruct> = NonNull::new(Box::into_raw(Box::new(InvalidIndexStruct { value: 7 }))).unwrap();
    let index: usize = usize::MAX; // Intentionally invalid index to trigger panic

    let bucket = unsafe { Bucket::from_base_index(base, index) };
    let ptr = bucket.as_ptr();
}

