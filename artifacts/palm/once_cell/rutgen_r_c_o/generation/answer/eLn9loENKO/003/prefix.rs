// Answer 0

#[test]
fn test_init_with_non_null_exchange() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::new();
    
    let result = once_box.init(|| {
        Ok(Box::new(TestStruct { value: 42 }))
    });

    assert!(result.is_ok());
    let value_ref = result.unwrap();
    assert_eq!(value_ref.value, 42);
    
    let old_value = Box::new(TestStruct { value: 100 });
    
    // Now set another value to cause compare_exchange to fail
    once_box.inner.store(Box::into_raw(old_value), Ordering::Release);
    
    let result_with_old = once_box.init(|| {
        Ok(Box::new(TestStruct { value: 84 }))
    });

    assert!(result_with_old.is_ok());
    let value_ref_with_old = result_with_old.unwrap();
    assert_eq!(value_ref_with_old.value, 42);
}

#[test]
fn test_init_with_err_function() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::new();
    
    let result = once_box.init(|| {
        Err("Error occurred")
    });

    assert!(result.is_err());
}

#[test]
fn test_init_with_multiple_initializations() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::new();

    let first_result = once_box.init(|| {
        Ok(Box::new(TestStruct { value: 10 }))
    });

    assert!(first_result.is_ok());
    assert_eq!(first_result.unwrap().value, 10);

    let second_result = once_box.init(|| {
        Ok(Box::new(TestStruct { value: 20 }))
    });

    assert!(second_result.is_ok());
    assert_eq!(second_result.unwrap().value, 10);
}

