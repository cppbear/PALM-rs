// Answer 0

#[test]
fn test_set_sensitive_true() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
}

#[test]
fn test_set_sensitive_false() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(false);
}

#[test]
fn test_set_sensitive_multiple_calls() {
    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    val.set_sensitive(false);
    val.set_sensitive(true);
}

