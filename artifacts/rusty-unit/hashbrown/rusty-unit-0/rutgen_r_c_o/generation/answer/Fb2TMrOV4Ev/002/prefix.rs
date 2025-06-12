// Answer 0

#[test]
fn test_next_n_non_zero_sized_type_offset_within_bounds() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    let base_ptr = NonNull::new(unsafe { &mut *(0 as *mut TestType) }).unwrap();
    let bucket = Bucket { ptr: base_ptr };
    let offset = 1; // Valid offset within bounds
    let new_bucket = unsafe { bucket.next_n(offset) };
}

#[test]
fn test_next_n_non_zero_sized_type_offset_at_bounds() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    let base_ptr = NonNull::new(unsafe { &mut *(0 as *mut TestType) }).unwrap();
    let bucket = Bucket { ptr: base_ptr };
    let offset = 10; // Assuming this is equal to RawTableInner.bucket_mask
    let new_bucket = unsafe { bucket.next_n(offset) }; 
}

#[test]
fn test_next_n_non_zero_sized_type_offset_below_bounds() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    let base_ptr = NonNull::new(unsafe { &mut *(0 as *mut TestType) }).unwrap();
    let bucket = Bucket { ptr: base_ptr };
    let offset = 5; // Valid offset below RawTableInner.bucket_mask
    let new_bucket = unsafe { bucket.next_n(offset) };
}

#[test]
#[should_panic]
fn test_next_n_non_zero_sized_type_offset_exceeds_bounds() {
    struct TestType;
    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }
    let base_ptr = NonNull::new(unsafe { &mut *(0 as *mut TestType) }).unwrap();
    let bucket = Bucket { ptr: base_ptr };
    let offset = 11; // Assuming this exceeds RawTableInner.bucket_mask
    let _new_bucket = unsafe { bucket.next_n(offset) }; 
}

