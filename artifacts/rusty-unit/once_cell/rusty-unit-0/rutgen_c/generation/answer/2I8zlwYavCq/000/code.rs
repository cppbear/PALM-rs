// Answer 0

#[cfg(test)]
fn test_compare_exchange_success() {
    struct TestStruct {
        value: usize,
    }
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    let test_value = TestStruct { value: 42 };
    
    match once_ref.compare_exchange(&test_value) {
        Ok(()) => {}
        Err(_) => panic!("Expected success but got an error"),
    }
}

#[cfg(test)]
fn test_compare_exchange_fail() {
    struct TestStruct {
        value: usize,
    }
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    let test_value = TestStruct { value: 42 };
    
    // First call should succeed
    let _ = once_ref.compare_exchange(&test_value).unwrap();
    
    // Second call should fail since it's already set
    match once_ref.compare_exchange(&test_value) {
        Ok(()) => panic!("Expected failure but got success"),
        Err(_) => {}
    }
}

#[cfg(test)]
fn test_compare_exchange_boundary() {
    struct TestStruct {
        value: usize,
    }
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    
    // Testing with a null pointer situation
    let null_value: *const TestStruct = std::ptr::null();
    match once_ref.compare_exchange(&null_value) {
        Ok(()) => {}
        Err(_) => panic!("Expected success but got an error with null pointer"),
    }
}

