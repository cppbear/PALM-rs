// Answer 0

#[test]
fn test_as_str_get() {
    let method = Method::GET;
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = Method::POST;
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = Method::PUT;
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = Method::DELETE;
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = Method::HEAD;
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_options() {
    let method = Method::OPTIONS;
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_trace() {
    let method = Method::TRACE;
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = Method::CONNECT;
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = Method::PATCH;
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    let inline_extension = InlineExtension::new(b"INLINE").unwrap();
    let method = Method(Inner::ExtensionInline(inline_extension));
    assert_eq!(method.as_str(), "INLINE");
}

#[test]
fn test_as_str_extension_allocated() {
    let allocated_extension = AllocatedExtension::new(b"ALLOCATED").unwrap();
    let method = Method(Inner::ExtensionAllocated(allocated_extension));
    assert_eq!(method.as_str(), "ALLOCATED");
}

