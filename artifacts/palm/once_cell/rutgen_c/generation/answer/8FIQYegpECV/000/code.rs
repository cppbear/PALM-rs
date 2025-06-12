// Answer 0

#[test]
fn test_set_success() {
    struct TestStruct {
        data: i32,
    }
    
    let once_box: OnceBox<TestStruct> = OnceBox::new();
    let result = once_box.set(Box::new(TestStruct { data: 42 }));
    
    assert!(result.is_ok());
}

#[test]
fn test_set_failure() {
    struct TestStruct {
        data: i32,
    }
    
    let once_box: OnceBox<TestStruct> = OnceBox::new();
    let _ = once_box.set(Box::new(TestStruct { data: 42 }));
    
    let result = once_box.set(Box::new(TestStruct { data: 43 }));
    
    assert!(result.is_err());
}

#[test]
fn test_set_boundary_condition() {
    struct TestStruct {
        data: i32,
    }
    
    let once_box: OnceBox<TestStruct> = OnceBox::new();
    let result = once_box.set(Box::new(TestStruct { data: 0 }));
    
    assert!(result.is_ok());
    let result = once_box.set(Box::new(TestStruct { data: -1 }));
    
    assert!(result.is_err());
}

