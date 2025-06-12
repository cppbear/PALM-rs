// Answer 0

#[test]
fn test_is_safe_options() {
    let method = Method::from_bytes(b"OPTIONS").unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_get() {
    let method = Method::from_bytes(b"GET").unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_head() {
    let method = Method::from_bytes(b"HEAD").unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_trace() {
    let method = Method::from_bytes(b"TRACE").unwrap();
    method.is_safe();
}

