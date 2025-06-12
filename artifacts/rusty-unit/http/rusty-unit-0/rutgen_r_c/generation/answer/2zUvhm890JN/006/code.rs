// Answer 0

#[test]
fn test_as_str_get() {
    let method = Method(Method::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = Method(Method::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = Method(Method::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = Method(Method::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = Method(Method::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_options() {
    let method = Method(Method::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_trace() {
    let method = Method(Method::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = Method(Method::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = Method(Method::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_inline_extension() {
    let inline_extension = InlineExtension::new(b"inline").unwrap();
    let method = Method(Method::ExtensionInline(inline_extension));
    assert_eq!(method.as_str(), "inline");
}

#[test]
fn test_as_str_allocated_extension() {
    let allocated_extension = AllocatedExtension::new(b"allocated").unwrap();
    let method = Method(Method::ExtensionAllocated(allocated_extension));
    assert_eq!(method.as_str(), "allocated");
}

