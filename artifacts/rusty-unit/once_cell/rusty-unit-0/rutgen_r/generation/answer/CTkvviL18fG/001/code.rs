// Answer 0

#[test]
fn test_get_or_init_success() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_multiple_calls() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let first_value = cell.get_or_init(|| 100);
    assert_eq!(first_value, &100);

    let second_value = cell.get_or_init(|| 200);
    assert_eq!(second_value, &100); // Should return the first initialized value
}

#[should_panic]
fn test_get_or_init_reentrant_initialization() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| {
        cell.get_or_init(|| 300); // Attempting to reinitialize should panic or deadlock
        400
    });
}

#[test]
fn test_get_or_init_panic_propagation() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = std::panic::catch_unwind(|| {
        cell.get_or_init(|| panic!("Initial function panicked"));
    });
    assert!(_value.is_err());
    
    // Ensure that the cell remains uninitialized
    assert!(cell.get().is_none());
}

