// Answer 0

#[test]
fn test_get_or_init_initialization() {
    struct TestData {
        value: i32,
    }

    let cell = OnceCell::new();
    
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_reentrance_panic() {
    struct TestData {
        value: i32,
    }

    let cell = OnceCell::new();

    let _ = cell.get_or_init(|| 38); // First call initializes.
    
    let panic_result = std::panic::catch_unwind(|| {
        cell.get_or_init(|| 17); // This should panic due to reentrant initialization
    });
    
    assert!(panic_result.is_err());
}

#[test]
fn test_get_or_init_with_panic_in_function() {
    struct TestData {
        value: i32,
    }

    let cell = OnceCell::new();
    
    let value = cell.get_or_init(|| panic!("This will panic"));
    assert_eq!(value, &0); // This will not get executed.
}

#[test]
fn test_get_or_init_with_previous_value() {
    struct TestData {
        value: i32,
    }

    let cell = OnceCell::new();
    
    let value = cell.get_or_init(|| 25);
    assert_eq!(value, &25);

    // Trying to reinitialize again should return the same value without panic
    let same_value = cell.get_or_init(|| 99);
    assert_eq!(same_value, &25); // Should still point to 25
}

