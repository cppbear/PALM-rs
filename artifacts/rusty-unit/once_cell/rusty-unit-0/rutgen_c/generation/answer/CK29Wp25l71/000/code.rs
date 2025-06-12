// Answer 0

#[test]
fn test_get_or_init_when_empty() {
    let once_bool = OnceBool::new();
    let result = once_bool.get_or_init(|| true);
    assert!(result);
}

#[test]
fn test_get_or_init_when_empty_false() {
    let once_bool = OnceBool::new();
    let result = once_bool.get_or_init(|| false);
    assert!(!result);
}

#[test]
fn test_get_or_init_with_multiple_calls() {
    let once_bool = OnceBool::new();
    let result1 = once_bool.get_or_init(|| true);
    let result2 = once_bool.get_or_init(|| false);
    assert!(result1);
    assert_eq!(result1, result2);
}

#[test]
fn test_get_or_init_with_predefined_value() {
    let once_bool = OnceBool::new();
    let value = once_bool.get_or_init(|| true);
    assert_eq!(value, true);
    let value_again = once_bool.get_or_init(|| false);
    assert_eq!(value_again, true);
}

