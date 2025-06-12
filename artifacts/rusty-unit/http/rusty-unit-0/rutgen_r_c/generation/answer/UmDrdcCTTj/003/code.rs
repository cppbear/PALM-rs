// Answer 0

#[test]
fn test_is_idempotent_put() {
    let method = Method(Method::Inner::Put);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_delete() {
    let method = Method(Method::Inner::Delete);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_get() {
    let method = Method(Method::Inner::Get);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_head() {
    let method = Method(Method::Inner::Head);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_options() {
    let method = Method(Method::Inner::OPTIONS);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_trace() {
    let method = Method(Method::Inner::Trace);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_connect() {
    let method = Method(Method::Inner::Connect);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_patch() {
    let method = Method(Method::Inner::Patch);
    assert_eq!(method.is_idempotent(), true);
}

