// Answer 0

#[derive(Debug)]
struct Method(String);

impl Method {
    fn is_safe(&self) -> bool {
        matches!(self.0.as_str(), "GET" | "HEAD" | "OPTIONS" | "TRACE")
    }
}

#[test]
fn test_is_safe_with_get() {
    let method = Method("GET".to_string());
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_head() {
    let method = Method("HEAD".to_string());
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_options() {
    let method = Method("OPTIONS".to_string());
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_trace() {
    let method = Method("TRACE".to_string());
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_with_invalid_method() {
    let method = Method("POST".to_string());
    assert!(!method.is_safe());
}

