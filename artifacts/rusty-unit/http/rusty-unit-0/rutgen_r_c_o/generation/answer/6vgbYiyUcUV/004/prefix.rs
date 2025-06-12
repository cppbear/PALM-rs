// Answer 0

#[test]
fn test_is_safe_get() {
    let method = Method(Method::GET);
    method.is_safe();
}

#[test]
fn test_is_safe_head() {
    let method = Method(Method::HEAD);
    method.is_safe();
}

#[test]
fn test_is_safe_trace() {
    let method = Method(Method::TRACE);
    method.is_safe();
}

#[test]
fn test_is_safe_options() {
    let method = Method(Method::OPTIONS);
    method.is_safe();
}

