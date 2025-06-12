// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_cell() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    // Initializing with a successful value
    let _ = cell.get_or_try_init(|| Ok(42));
    
    // Calling get_or_try_init when cell is already initialized
    let result = cell.get_or_try_init(|| Err(()));
    
    assert_eq!(result, Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_with_error_return() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    
    // Testing when initialization returns an error
    let result = cell.get_or_try_init(|| Err(()));
    
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none());
}

