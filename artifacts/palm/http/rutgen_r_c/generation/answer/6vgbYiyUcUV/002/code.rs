// Answer 0

#[test]
fn test_is_safe_with_get_method() {
    let method = Method(Method::GET);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_head_method() {
    let method = Method(Method::HEAD);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_options_method() {
    let method = Method(Method::OPTIONS);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_trace_method() {
    let method = Method(Method::TRACE);
    assert!(method.is_safe());
}

