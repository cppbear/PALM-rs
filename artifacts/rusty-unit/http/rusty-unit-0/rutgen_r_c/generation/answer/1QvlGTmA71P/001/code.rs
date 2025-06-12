// Answer 0

#[test]
fn test_is_sensitive_initial_state() {
    let val = HeaderValue::from_static("not sensitive");
    assert!(!val.is_sensitive());
}

#[test]
fn test_is_sensitive_after_setting_true() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_is_sensitive_after_setting_false() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

#[test]
fn test_is_sensitive_with_empty_string() {
    let val = HeaderValue::from_static("");
    assert!(!val.is_sensitive());
}

#[test]
fn test_is_sensitive_with_large_string() {
    let mut val = HeaderValue::from_static("some large sensitive data string that continues for a while...");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_is_sensitive_after_multiple_sets() {
    let mut val = HeaderValue::from_static("example");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

