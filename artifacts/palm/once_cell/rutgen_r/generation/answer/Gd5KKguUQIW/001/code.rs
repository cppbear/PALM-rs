// Answer 0

#[test]
fn test_deref_mut_with_initialized_lazy() {
    use once_cell::sync::Lazy;

    struct TestStruct {
        value: i32,
    }

    let mut lazy_value = Lazy::new(|| TestStruct { value: 42 });

    let result = lazy_value.deref_mut();

    assert_eq!(result.value, 42);
}

#[test]
fn test_deref_mut_with_uninitialized_lazy() {
    use once_cell::sync::Lazy;

    struct TestStruct {
        value: i32,
    }

    let mut lazy_value: Lazy<TestStruct> = Lazy::new(|| TestStruct { value: 0 });

    // First call to deref_mut should initialize and return mutable reference
    let result = lazy_value.deref_mut();

    result.value = 100;

    assert_eq!(result.value, 100);
}

#[should_panic]
#[test]
fn test_deref_mut_on_double_mutable_borrow() {
    use once_cell::sync::Lazy;

    struct TestStruct {
        value: i32,
    }

    let mut lazy_value = Lazy::new(|| TestStruct { value: 0 });

    // Obtain a mutable reference without initialization
    let first_borrow = lazy_value.deref_mut();
    
    // Attempt to create a second mutable borrow which should panic
    let _second_borrow = lazy_value.deref_mut(); // This should panic
}

