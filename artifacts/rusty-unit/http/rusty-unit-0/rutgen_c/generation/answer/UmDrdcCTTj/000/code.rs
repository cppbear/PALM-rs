// Answer 0

#[test]
fn test_is_idempotent_with_put() {
    let method = Method(Method::Put);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_delete() {
    let method = Method(Method::Delete);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_get() {
    let method = Method(Method::Get);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_head() {
    let method = Method(Method::HEAD);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_options() {
    let method = Method(Method::OPTIONS);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_trace() {
    let method = Method(Method::TRACE);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_patch() {
    let method = Method(Method::PATCH);
    assert!(!method.is_idempotent());
}

#[test]
fn test_is_idempotent_with_connect() {
    let method = Method(Method::CONNECT);
    assert!(!method.is_idempotent());
}

