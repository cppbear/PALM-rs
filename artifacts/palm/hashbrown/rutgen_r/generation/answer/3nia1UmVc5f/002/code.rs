// Answer 0

#[test]
fn test_from_base_index_non_zero_sized() {
    use std::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut data: [TestType; 4] = Default::default(); // create an array of 4 TestType elements
    let base_ptr = NonNull::new(&mut data[4] as *mut TestType).expect("Base pointer should not be null");
    
    // Test valid index
    let index = 1;
    let bucket = unsafe { from_base_index(base_ptr, index) };
    assert_eq!(bucket.ptr.as_ptr(), &mut data[4 - index] as *mut TestType);

    // Test boundary index (0 is valid)
    let index = 0;
    let bucket = unsafe { from_base_index(base_ptr, index) };
    assert_eq!(bucket.ptr.as_ptr(), &mut data[4 - index] as *mut TestType);

    // Test maximum index (bucket_mask is the last valid index, which is 3 in this case)
    let index = 3;
    let bucket = unsafe { from_base_index(base_ptr, index) };
    assert_eq!(bucket.ptr.as_ptr(), &mut data[4 - index] as *mut TestType);
}

#[should_panic]
#[test]
fn test_from_base_index_too_large_index() {
    use std::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut data: [TestType; 4] = Default::default(); // create an array of 4 TestType elements
    let base_ptr = NonNull::new(&mut data[4] as *mut TestType).expect("Base pointer should not be null");

    // This should panic because index is greater than bucket_mask (which is 3)
    let index = 4;
    let _bucket = unsafe { from_base_index(base_ptr, index) };
}

