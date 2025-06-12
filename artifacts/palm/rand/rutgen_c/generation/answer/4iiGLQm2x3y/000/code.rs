// Answer 0

#[test]
fn test_bool_any_true() {
    let value = true;
    assert_eq!(value.any(), true);
}

#[test]
fn test_bool_any_false() {
    let value = false;
    assert_eq!(value.any(), false);
}

