// Answer 0

#[test]
fn test_get_or_init_success() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| TestStruct { value: 42 });
    assert_eq!(value.value, 42);
}

#[test]
#[should_panic]
fn test_get_or_init_panic() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| {
        panic!("This panic is expected");
    });
}

#[test]
fn test_get_or_init_reentrant_panic() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::new();
    let _value1 = cell.get_or_init(|| TestStruct { value: 42 });
    
    // Reentrant call to trigger panic
    let _value2 = cell.get_or_init(|| {
        let _ = cell.get_or_init(|| TestStruct { value: 55 });
        100 // This value will not be reached
    });
}

#[test]
fn test_get_or_init_multiple_calls() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::new();
    let value1 = cell.get_or_init(|| TestStruct { value: 100 });
    assert_eq!(value1.value, 100);

    // The second call should retrieve the already initialized value
    let value2 = cell.get_or_init(|| TestStruct { value: 200 });
    assert_eq!(value2.value, 100); // should still be the first value
}

#[test]
fn test_get_or_init_intermediate_init_success() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::new();
    
    // First call initializes successfully
    {
        let value = cell.get_or_init(|| TestStruct { value: 500 });
        assert_eq!(value.value, 500);
    }
    
    // Second call should return the initialized value without re-initializing
    {
        let value = cell.get_or_init(|| TestStruct { value: 600 });
        assert_eq!(value.value, 500);
    }
}

