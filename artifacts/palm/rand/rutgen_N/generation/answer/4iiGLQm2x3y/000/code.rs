// Answer 0

#[test]
fn test_any_true() {
    let value: bool = true;
    assert_eq!(value.any(), true);
}

#[test]
fn test_any_false() {
    let value: bool = false;
    assert_eq!(value.any(), false);
}

