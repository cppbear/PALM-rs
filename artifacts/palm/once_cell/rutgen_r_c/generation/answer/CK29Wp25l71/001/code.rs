// Answer 0

#[test]
fn test_get_or_init_empty() {
    let once_bool = OnceBool::new();
    let result = once_bool.get_or_init(|| true);
    assert!(result);
    assert_eq!(once_bool.get(), Some(true));
}

#[test]
fn test_get_or_init_non_empty() {
    let once_bool = OnceBool::new();
    let _ = once_bool.get_or_init(|| false);
    let result = once_bool.get_or_init(|| true);
    assert!(!result);
    assert_eq!(once_bool.get(), Some(true));
}

#[test]
fn test_get_or_init_panic_conditions() {
    let once_bool = OnceBool::new();
    let result = std::panic::catch_unwind(|| {
        let _ = once_bool.get_or_init(|| panic!("test panic"));
    });
    assert!(result.is_err());
}

#[test]
fn test_get_or_init_same_value() {
    let once_bool = OnceBool::new();
    let _ = once_bool.get_or_init(|| true);
    let result = once_bool.get_or_init(|| false);
    assert!(!result);
    assert_eq!(once_bool.get(), Some(true));
}

