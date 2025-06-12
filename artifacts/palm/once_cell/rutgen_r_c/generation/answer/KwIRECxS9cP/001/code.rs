// Answer 0

#[test]
fn test_get_or_try_init_with_existing_value() {
    use alloc::boxed::Box;

    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::with_value(Box::new(TestStruct { value: 42 }));

    let result = once_box.get_or_try_init(|| {
        // This function should not be called since the value already exists.
        Ok(Box::new(TestStruct { value: 100 }))
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_get_or_try_init_with_value_not_existing() {
    use alloc::boxed::Box;

    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::new();

    let result = once_box.get_or_try_init(|| {
        // This function will be called to initialize the value.
        Ok(Box::new(TestStruct { value: 99 }))
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 99);
}

