// Answer 0

#[test]
fn test_get_or_init_with_valid_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_with_no_side_effects() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 100);
    assert_eq!(value, &100);
    let value_again = cell.get_or_init(|| 200);
    assert_eq!(value_again, &100); // Should still return the first initialization.
}

#[should_panic]
fn test_get_or_init_with_panic_in_initializer() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("Initializer panicked"));
}

#[should_panic]
fn test_get_or_init_with_reentrant_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| cell.get_or_init(|| 123)); // Reentrant call should panic.
}

