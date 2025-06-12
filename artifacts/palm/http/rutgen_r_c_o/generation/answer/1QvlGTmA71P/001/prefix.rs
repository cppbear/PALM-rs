// Answer 0

#[test]
fn test_is_sensitive_true() {
    let mut val = HeaderValue::from_static("sensitive data");
    val.set_sensitive(true);
    let _ = val.is_sensitive();
}

#[test]
fn test_is_sensitive_false() {
    let mut val = HeaderValue::from_static("not sensitive data");
    val.set_sensitive(false);
    let _ = val.is_sensitive();
}

#[test]
fn test_is_sensitive_edge_case() {
    let mut val = HeaderValue::from_static("toggle sensitive state");
    val.set_sensitive(true);
    let _ = val.is_sensitive();
    val.set_sensitive(false);
    let _ = val.is_sensitive();
}

