// Answer 0

#[test]
fn test_get_or_try_init_with_empty_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let result = cell.get_or_try_init(|| Err(()));
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_with_successful_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let result = cell.get_or_try_init(|| Ok(42));
    assert_eq!(result, Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic(expected = "reentrant init")]
fn test_get_or_try_init_with_reentrant_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    
    let _ = cell.get_or_try_init(|| {
        let _ = cell.get_or_try_init(|| Ok(42)); // This will panic
        Ok(0)
    });
}

