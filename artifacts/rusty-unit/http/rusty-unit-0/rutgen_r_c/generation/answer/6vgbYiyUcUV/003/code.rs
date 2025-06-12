// Answer 0

#[test]
fn test_is_safe_get() {
    let method = Method(Method::Get);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_head() {
    let method = Method(Method::Head);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_options() {
    let method = Method(Method::Options);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_trace() {
    let method = Method(Method::Trace);
    assert!(method.is_safe());
}

