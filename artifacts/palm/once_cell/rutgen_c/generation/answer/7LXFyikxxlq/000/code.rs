// Answer 0

#[test]
fn test_get_or_try_init_value_not_set() {
    struct TestData;
    let once_ref = OnceRef::new();

    let result = once_ref.get_or_try_init(|| Ok(&TestData));
    
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap(), &TestData);
}

#[test]
fn test_get_or_try_init_value_set() {
    struct TestData;
    let once_ref = OnceRef::new();
    
    // Initialize it first.
    let _ = once_ref.get_or_try_init(|| Ok(&TestData));
    let result = once_ref.get_or_try_init(|| Ok(&TestData));
    
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap(), &TestData);
}

#[should_panic]
fn test_get_or_try_init_with_error() {
    struct TestData;
    let once_ref = OnceRef::new();
    
    let _ = once_ref.get_or_try_init(|| Err("error"));
    let result = once_ref.get_or_try_init(|| Err("error"));
    
    assert!(result.is_err());
} 

#[test]
fn test_get_or_try_init_multiple_initializations() {
    struct TestData;
    let once_ref = OnceRef::new();
    
    let result1 = once_ref.get_or_try_init(|| Ok(&TestData));
    let result2 = once_ref.get_or_try_init(|| Ok(&TestData));
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.as_ref().unwrap(), result2.as_ref().unwrap());
}

