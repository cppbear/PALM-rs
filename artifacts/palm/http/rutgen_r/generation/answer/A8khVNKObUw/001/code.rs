// Answer 0

#[test]
fn test_set_sensitive_true() {
    use http::header::HeaderValue;

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_set_sensitive_false() {
    use http::header::HeaderValue;

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

#[test]
fn test_set_sensitive_multiple_changes() {
    use http::header::HeaderValue;

    let mut val = HeaderValue::from_static("my secret");
    
    val.set_sensitive(true);
    assert!(val.is_sensitive());

    val.set_sensitive(false);
    assert!(!val.is_sensitive());

    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

