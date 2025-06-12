// Answer 0

#[test]
fn test_as_str_options() {
    let method = Method(Method::OPTIONS);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = Method(Method::GET);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = Method(Method::POST);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = Method(Method::PUT);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = Method(Method::DELETE);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = Method(Method::HEAD);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = Method(Method::TRACE);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = Method(Method::CONNECT);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = Method(Method::PATCH);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_inline_extension() {
    let inline_extension = InlineExtension::new(b"INLINE").unwrap();
    let method = Method(Method::ExtensionInline(inline_extension));
    assert_eq!(method.as_str(), "INLINE");
}

#[test]
fn test_as_str_allocated_extension() {
    let allocated_extension = AllocatedExtension::new(b"ALLOCATED").unwrap();
    let method = Method(Method::ExtensionAllocated(allocated_extension));
    assert_eq!(method.as_str(), "ALLOCATED");
}

