// Answer 0


use once_cell::sync::OnceCell;

#[test]
fn test_get_or_try_init_with_initialized_value() {
    let cell = OnceCell::new();

    // Initialize the cell with a value successfully
    let result = cell.get_or_try_init(|| Ok(42));
    assert_eq!(result, Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_with_error() {
    let cell = OnceCell::new();

    // Attempt to initialize the cell but return an error
    let result = cell.get_or_try_init(|| Err("Initialization failed"));
    assert_eq!(result, Err("Initialization failed"));
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_when_initialized() {
    let cell = OnceCell::new();
    // First successful initialization
    cell.get_or_try_init(|| Ok(100)).unwrap();

    // Attempting to re-initialize should yield the existing value
    let new_result = cell.get_or_try_init(|| Ok(200));
    assert_eq!(new_result, Ok(&100));
    assert_eq!(cell.get(), Some(&100));
}

#[test]
#[should_panic]
fn test_get_or_try_init_panics_on_f_panic() {
    let cell = OnceCell::new();

    // This closure will panic
    cell.get_or_try_init(|| panic!("Intentional panic"));
}

#[test]
fn test_get_or_try_init_panics_on_reentrant_init() {
    let cell = OnceCell::new();

    // Create a closure for initialization that tries to call the function again
    let result = cell.get_or_try_init(|| {
        // This will attempt to initialize again, which is a reentrant call
        cell.get_or_try_init(|| Ok(50)).unwrap(); 
        Ok(25)
    });

    assert_eq!(result, Ok(&25));
    assert_eq!(cell.get(), Some(&25));
}


