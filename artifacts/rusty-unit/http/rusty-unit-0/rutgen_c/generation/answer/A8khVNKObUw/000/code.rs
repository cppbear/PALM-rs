// Answer 0

#[test]
fn test_set_sensitive() {
    let mut val = HeaderValue {
        inner: Bytes::from_static(b"my secret"),
        is_sensitive: false,
    };

    val.set_sensitive(true);
    assert!(val.is_sensitive);

    val.set_sensitive(false);
    assert!(!val.is_sensitive);
}

#[test]
fn test_set_sensitive_initial_state() {
    let mut val = HeaderValue {
        inner: Bytes::from_static(b"my secret"),
        is_sensitive: false,
    };

    assert!(!val.is_sensitive);
}

