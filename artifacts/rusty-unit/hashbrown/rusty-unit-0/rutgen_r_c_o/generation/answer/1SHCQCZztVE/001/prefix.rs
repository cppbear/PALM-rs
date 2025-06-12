// Answer 0

#[test]
fn test_as_mut_non_zero_sized_valid() {
    struct TestType {
        value: i32,
    }
    
    let base_value = Box::new(TestType { value: 42 });
    let base_non_null = NonNull::new(Box::into_raw(base_value)).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base_non_null, 1) };
    
    unsafe {
        let result: &mut TestType = bucket.as_mut();
        *result = TestType { value: 100 };
    }
}

#[test]
#[should_panic]
fn test_as_mut_zero_sized() {
    struct ZeroSized;

    let base_value = Box::new(ZeroSized);
    let base_non_null = NonNull::new(Box::into_raw(base_value)).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base_non_null, 0) };

    unsafe {
        let _result: &mut ZeroSized = bucket.as_mut(); 
    } 
}

#[test]
fn test_as_mut_non_zero_sized_boundary() {
    struct BoundaryType {
        value: i32,
    }

    let base_value = Box::new(BoundaryType { value: 0 });
    let base_non_null = NonNull::new(Box::into_raw(base_value)).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base_non_null, usize::MAX) }; 

    unsafe {
        let result: &mut BoundaryType = bucket.as_mut();
        *result = BoundaryType { value: 2023 };
    }
}

#[test]
fn test_as_mut_with_different_values() {
    struct DifferentType {
        value: i32,
    }

    let base_value = Box::new(DifferentType { value: 500 });
    let base_non_null = NonNull::new(Box::into_raw(base_value)).unwrap();
    let bucket = unsafe { Bucket::from_base_index(base_non_null, 2) };

    unsafe {
        let result: &mut DifferentType = bucket.as_mut();
        *result = DifferentType { value: 999 };
    }
}

