// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_cell() {
    struct TestValue(i32);
    
    let cell = OnceCell::with_value(TestValue(42));
    let result = cell.get_or_try_init(|| Ok(TestValue(100)));
    
    assert_eq!(result, Ok(&TestValue(42)));
}

#[test]
fn test_get_or_try_init_with_initialization_error() {
    struct TestValue(i32);
    
    let cell = OnceCell::new();
    let result: Result<&TestValue, ()> = cell.get_or_try_init(|| Err(()));
    
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_with_successful_initialization() {
    struct TestValue(i32);
    
    let cell = OnceCell::new();
    
    let result: Result<&TestValue, ()> = cell.get_or_try_init(|| Ok(TestValue(84)));
    
    assert_eq!(result, Ok(&TestValue(84)));
    assert_eq!(cell.get(), Some(&TestValue(84)));
}

#[test]
#[should_panic]
fn test_get_or_try_init_panics_on_reentrant_initialization() {
    struct ReentrantValue(i32);
    
    let cell = OnceCell::new();
    
    // This function will call `get_or_try_init` again, resulting in panic
    let _result = cell.get_or_try_init(|| {
        cell.get_or_try_init(|| Ok(ReentrantValue(77)));
        Ok(ReentrantValue(50))
    });
}

