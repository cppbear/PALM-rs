// Answer 0

#[derive(Debug)]
struct MethodWrapper(Method);

#[derive(Debug)]
enum Method {
    Get,
    Head,
    Options,
    Trace,
    Post, // Not safe
    Put,  // Not safe
}

impl MethodWrapper {
    pub fn is_safe(&self) -> bool {
        matches!(self.0, Method::Get | Method::Head | Method::Options | Method::Trace)
    }
}

#[test]
fn test_is_safe_get() {
    let method = MethodWrapper(Method::Get);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_head() {
    let method = MethodWrapper(Method::Head);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_options() {
    let method = MethodWrapper(Method::Options);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_trace() {
    let method = MethodWrapper(Method::Trace);
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_post() {
    let method = MethodWrapper(Method::Post);
    assert!(!method.is_safe());
}

#[test]
fn test_is_safe_put() {
    let method = MethodWrapper(Method::Put);
    assert!(!method.is_safe());
}

