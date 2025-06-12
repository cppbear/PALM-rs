// Answer 0

#[test]
fn test_is_idempotent_delete() {
    let method = Method::DELETE;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_put() {
    let method = Method::PUT;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_get() {
    let method = Method::GET;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_head() {
    let method = Method::HEAD;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_options() {
    let method = Method::OPTIONS;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_trace() {
    let method = Method::TRACE;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_connect() {
    let method = Method::CONNECT;
    method.is_idempotent();
}

#[test]
fn test_is_idempotent_patch() {
    let method = Method::PATCH;
    method.is_idempotent();
}

