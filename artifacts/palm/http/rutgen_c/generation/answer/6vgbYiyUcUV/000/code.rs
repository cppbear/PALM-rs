// Answer 0

#[test]
fn test_is_safe_get() {
    let method = Method::GET;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_head() {
    let method = Method::HEAD;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_options() {
    let method = Method::OPTIONS;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_trace() {
    let method = Method::TRACE;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_post() {
    let method = Method::POST;
    assert!(!method.is_safe());
}

#[test]
fn test_is_safe_put() {
    let method = Method::PUT;
    assert!(!method.is_safe());
}

#[test]
fn test_is_safe_delete() {
    let method = Method::DELETE;
    assert!(!method.is_safe());
}

#[test]
fn test_is_safe_patch() {
    let method = Method::PATCH;
    assert!(!method.is_safe());
}

#[test]
fn test_is_safe_connect() {
    let method = Method::CONNECT;
    assert!(!method.is_safe());
}

