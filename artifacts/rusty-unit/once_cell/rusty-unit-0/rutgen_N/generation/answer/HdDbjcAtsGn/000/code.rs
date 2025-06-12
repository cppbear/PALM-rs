// Answer 0

#[test]
fn test_get_or_init_initializes_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 92);
    assert_eq!(value, &92);
}

#[test]
fn test_get_or_init_returns_existing_value() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 92);
    assert_eq!(value, &92);
    let value = cell.get_or_init(|| 100);
    assert_eq!(value, &92);
}

#[should_panic]
fn test_get_or_init_panics_on_reentrant_init() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _ = cell.get_or_init(|| {
        cell.get_or_init(|| 92);
        100
    });
}

#[should_panic]
fn test_get_or_init_panics_if_f_panics() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    let _ = cell.get_or_init(|| panic!("intentional panic"));
}

