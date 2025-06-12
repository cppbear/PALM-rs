// Answer 0

#[test]
fn test_get_or_init_with_initialization() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 92);
    assert_eq!(value, &92);
}

#[test]
fn test_get_or_init_without_reinitialization() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 92);
    assert_eq!(value, &92);
    
    let value_again = cell.get_or_init(|| unreachable!());
    assert_eq!(value_again, &92);
}

#[should_panic]
fn test_get_or_init_panics_on_initialization_failure() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("Initialization panic!"));
}

