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
fn test_as_str_extension_inline() {
    let inline_ext = InlineExtension::new(b"EXTENSION").unwrap();
    let method = Method(Inner::ExtensionInline(inline_ext));
    assert_eq!(method.as_str(), "EXTENSION");
}

#[test]
fn test_as_str_extension_allocated() {
    let allocated_ext = AllocatedExtension::new(b"ALLOCATED_EXTENSION").unwrap();
    let method = Method(Inner::ExtensionAllocated(allocated_ext));
    assert_eq!(method.as_str(), "ALLOCATED_EXTENSION");
}

