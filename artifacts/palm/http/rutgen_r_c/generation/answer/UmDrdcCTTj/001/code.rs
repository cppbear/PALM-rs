// Answer 0

#[test]
fn test_is_idempotent_get() {
    let method = Method(Method::GET.clone());
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_head() {
    let method = Method(Method::HEAD.clone());
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_options() {
    let method = Method(Method::OPTIONS.clone());
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_trace() {
    let method = Method(Method::TRACE.clone());
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_patch() {
    let method = Method(Method::PATCH.clone());
    assert_eq!(method.is_idempotent(), false);
}

#[test]
fn test_is_idempotent_post() {
    let method = Method(Method::POST.clone());
    assert_eq!(method.is_idempotent(), false);
}

#[test]
fn test_is_idempotent_connect() {
    let method = Method(Method::CONNECT.clone());
    assert_eq!(method.is_idempotent(), false);
}

