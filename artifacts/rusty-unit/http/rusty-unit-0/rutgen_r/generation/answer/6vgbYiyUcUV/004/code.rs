// Answer 0

struct MethodWrapper(Method);

#[derive(Debug)]
enum Method {
    Get,
    Head,
    Options,
    Trace,
    Post, // for the purpose of testing, this is not safe
}

#[test]
fn test_is_safe_get() {
    let method_wrapper = MethodWrapper(Method::Get);
    assert!(method_wrapper.is_safe());
}

#[test]
fn test_is_safe_head() {
    let method_wrapper = MethodWrapper(Method::Head);
    assert!(method_wrapper.is_safe());
}

#[test]
fn test_is_safe_options() {
    let method_wrapper = MethodWrapper(Method::Options);
    assert!(method_wrapper.is_safe());
}

#[test]
fn test_is_safe_trace() {
    let method_wrapper = MethodWrapper(Method::Trace);
    assert!(method_wrapper.is_safe());
}

#[test]
fn test_is_safe_post() {
    let method_wrapper = MethodWrapper(Method::Post);
    assert!(!method_wrapper.is_safe());
}

