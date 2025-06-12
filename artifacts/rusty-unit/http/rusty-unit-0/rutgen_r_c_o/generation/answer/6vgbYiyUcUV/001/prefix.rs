// Answer 0

#[test]
fn test_is_safe_with_put() {
    let method = Method(Inner::Put);
    method.is_safe();
}

#[test]
fn test_is_safe_with_post() {
    let method = Method(Inner::Post);
    method.is_safe();
}

#[test]
fn test_is_safe_with_delete() {
    let method = Method(Inner::Delete);
    method.is_safe();
}

#[test]
fn test_is_safe_with_connect() {
    let method = Method(Inner::Connect);
    method.is_safe();
}

#[test]
fn test_is_safe_with_patch() {
    let method = Method(Inner::Patch);
    method.is_safe();
}

