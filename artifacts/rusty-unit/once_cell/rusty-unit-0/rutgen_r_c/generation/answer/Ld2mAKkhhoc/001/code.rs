// Answer 0

#[test]
fn test_set_when_full() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::with_value(TestStruct { value: 10 });
    
    let result = cell.set(TestStruct { value: 20 });
    
    assert_eq!(result, Err(TestStruct { value: 20 }));
}

#[test]
fn test_set_empty() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();
    
    let result = cell.set(TestStruct { value: 100 });
    
    assert_eq!(result, Ok(()));
} 

#[test]
fn test_set_return_previous_value() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();
    assert_eq!(cell.set(TestStruct { value: 50 }), Ok(()));
    
    let result = cell.set(TestStruct { value: 150 });
    
    assert_eq!(result, Err(TestStruct { value: 150 }));
} 

#[test]
fn test_set_repeatedly() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();

    assert_eq!(cell.set(TestStruct { value: 30 }), Ok(()));
    assert_eq!(cell.set(TestStruct { value: 40 }), Err(TestStruct { value: 40 }));
}

