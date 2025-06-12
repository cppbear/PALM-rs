// Answer 0

#[test]
fn test_get_with_not_null_pointer() {
    struct TestStruct {
        value: i32,
    }
    
    let once_box = OnceBox::with_value(Box::new(TestStruct { value: 42 }));
    let result = once_box.get();
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_get_with_null_pointer() {
    let once_box: OnceBox<i32> = OnceBox::new();
    let result = once_box.get();
    
    assert!(result.is_none());
}

