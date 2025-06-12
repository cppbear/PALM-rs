// Answer 0

#[test]
fn test_from_base_index_non_zero_sized() {
    struct NonZeroSized;
    
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSized))) 
        .expect("Failed to create NonNull");
    
    let bucket = unsafe { Bucket::<NonZeroSized>::from_base_index(base, 1) };
    
    assert_eq!(bucket.as_ptr() as *const NonZeroSized, base.as_ptr().sub(1));
}

#[test]
#[should_panic]
fn test_from_base_index_non_zero_sized_out_of_bounds() {
    struct NonZeroSized;
    
    impl NonZeroSized {
        const IS_ZERO_SIZED: bool = false;
    }

    let base = NonNull::new(Box::into_raw(Box::new(NonZeroSized))) 
        .expect("Failed to create NonNull");

    // Using an out of bounds index to test panic
    unsafe { Bucket::<NonZeroSized>::from_base_index(base, 100) };
}

#[test]
fn test_from_base_index_zero_sized() {
    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new(Box::into_raw(Box::new(ZeroSized))) 
        .expect("Failed to create NonNull");
    
    let bucket = unsafe { Bucket::<ZeroSized>::from_base_index(base, 1) };
    
    assert_eq!(bucket.as_ptr() as usize, (1 as *mut ZeroSized) as usize);
}

#[test]
#[should_panic]
fn test_from_base_index_zero_sized_out_of_bounds() {
    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new(Box::into_raw(Box::new(ZeroSized))) 
        .expect("Failed to create NonNull");

    // Using an out of bounds index to test panic
    unsafe { Bucket::<ZeroSized>::from_base_index(base, 100) };
}

