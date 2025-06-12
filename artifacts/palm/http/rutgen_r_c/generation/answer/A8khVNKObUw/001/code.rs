// Answer 0

#[test]
fn test_set_sensitive_true() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_set_sensitive_false() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

#[test]
fn test_set_sensitive_multiple_calls() {
    let mut val = HeaderValue::from_static("another secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

