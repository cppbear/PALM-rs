// Answer 0

#[test]
fn test_get_or_init_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_reinitialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _ = cell.get_or_init(|| 72);
    let value = cell.get_or_init(|| unreachable!());
    assert_eq!(value, &72);
}

#[should_panic]
fn test_get_or_init_reentrant_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _ = cell.get_or_init(|| {
        cell.get_or_init(|| 100);
        200
    });
}

#[should_panic]
fn test_get_or_init_function_panic() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _ = cell.get_or_init(|| panic!("This function panics"));
}

