// Answer 0

#[test]
fn test_as_non_null_with_valid_pointer() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_ptr: NonNull<TestType> = unsafe { NonNull::new_unchecked(0x1 as *mut TestType) };
    let bucket = Bucket::<TestType>::from_base_index(base_ptr, 0);
    let _ = bucket.as_non_null();
}

#[test]
fn test_as_non_null_with_zero_sized_type() {
    struct ZeroSized;
    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }
    
    let base_ptr: NonNull<ZeroSized> = unsafe { NonNull::new_unchecked(0x1 as *mut ZeroSized) };
    let bucket = Bucket::<ZeroSized>::from_base_index(base_ptr, 0);
    let _ = bucket.as_non_null();
}

#[test]
fn test_as_non_null_with_high_pointer_value() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_ptr: NonNull<TestType> = unsafe { NonNull::new_unchecked(0xFFFFFFFFFFFFFFFF as *mut TestType) };
    let bucket = Bucket::<TestType>::from_base_index(base_ptr, 0);
    let _ = bucket.as_non_null();
}

#[test]
fn test_as_non_null_with_non_zero_index() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_ptr: NonNull<TestType> = unsafe { NonNull::new_unchecked(0x1 as *mut TestType) };
    let bucket = Bucket::<TestType>::from_base_index(base_ptr, 5);
    let _ = bucket.as_non_null();
}

#[test]
fn test_as_non_null_with_offset() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    
    let base_ptr: NonNull<TestType> = unsafe { NonNull::new_unchecked(0x1 as *mut TestType) };
    let mut bucket = Bucket::<TestType>::from_base_index(base_ptr, 0);
    bucket = unsafe { bucket.next_n(5) };
    let _ = bucket.as_non_null();
}

