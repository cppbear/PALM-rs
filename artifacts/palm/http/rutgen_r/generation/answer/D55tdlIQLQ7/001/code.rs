// Answer 0

#[test]
fn test_into_any() {
    use std::any::Any;

    struct TestStruct;
    
    let test_instance = Box::new(TestStruct);
    let any_box: Box<dyn Any> = test_instance.into_any();
    
    // Assert that we can downcast it back to the original type
    let downcasted_instance = any_box.downcast::<TestStruct>();
    assert!(downcasted_instance.is_ok());
}

#[test]
#[should_panic]
fn test_into_any_panic() {
    // This test doesn't really trigger a panic from the `into_any` method itself since it can't panic.
    // However, we can simulate a panic scenario by trying to downcast to a wrong type.
    use std::any::Any;

    struct TestStruct;
    
    let test_instance = Box::new(TestStruct);
    let any_box: Box<dyn Any> = test_instance.into_any();
    
    // This should panic, as we are trying to downcast to an incorrect type
    let _ = any_box.downcast::<i32>().unwrap();
}

