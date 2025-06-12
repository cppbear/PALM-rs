// Answer 0

#[test]
fn test_is_sensitive_initially_false() {
    let val = HeaderValue {
        inner: Bytes::from_static(b"example"),
        is_sensitive: false,
    };
    assert!(!val.is_sensitive());
}

#[test]
fn test_is_sensitive_after_setting_true() {
    let mut val = HeaderValue {
        inner: Bytes::from_static(b"example"),
        is_sensitive: false,
    };
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_is_sensitive_after_setting_false() {
    let mut val = HeaderValue {
        inner: Bytes::from_static(b"example"),
        is_sensitive: true,
    };
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

#[test]
fn test_is_sensitive_stays_same() {
    let val = HeaderValue {
        inner: Bytes::from_static(b"example"),
        is_sensitive: true,
    };
    assert!(val.is_sensitive());
    let val_clone = val.clone();
    assert!(val_clone.is_sensitive());
}

