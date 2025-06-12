// Answer 0

#[test]
fn test_get_or_init_success() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 92);
    assert_eq!(value, &92);
}

#[test]
#[should_panic]
fn test_get_or_init_panic_on_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("Initialization panic!"));
}

#[test]
fn test_get_or_init_after_initialization() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value_first = cell.get_or_init(|| 42);
    assert_eq!(value_first, &42);

    let value_second = cell.get_or_init(|| unreachable!());
    assert_eq!(value_second, &42);
}

#[test]
#[should_panic]
fn test_get_or_init_reentrant_panic() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| {
        cell.get_or_init(|| panic!("Reentrant panic!"));
        100
    });
}

