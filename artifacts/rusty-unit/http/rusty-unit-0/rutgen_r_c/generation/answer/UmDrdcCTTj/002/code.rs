// Answer 0

#[test]
fn test_is_idempotent_delete() {
    let method = Method(Inner::Delete);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_put() {
    let method = Method(Inner::Put);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_get() {
    let method = Method(Inner::Get);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_head() {
    let method = Method(Inner::HEAD);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_options() {
    let method = Method(Inner::OPTIONS);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_trace() {
    let method = Method(Inner::TRACE);
    assert!(method.is_idempotent());
}

