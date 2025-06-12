// Answer 0

#[test]
fn test_method_is_safe_options() {
    let method = Method(Method::Options);
    assert!(method.is_safe());
}

#[test]
fn test_method_is_safe_head() {
    let method = Method(Method::HEAD);
    assert!(method.is_safe());
}

#[test]
fn test_method_is_safe_get() {
    let method = Method(Method::GET);
    assert!(method.is_safe());
}

#[test]
fn test_method_is_safe_trace() {
    let method = Method(Method::TRACE);
    assert!(method.is_safe());
}

