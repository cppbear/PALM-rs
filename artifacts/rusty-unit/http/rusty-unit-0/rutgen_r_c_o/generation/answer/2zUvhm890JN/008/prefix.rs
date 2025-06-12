// Answer 0

#[test]
fn test_as_str_put() {
    let method = Method(Method::Put);
    let result = method.as_str();
}

#[test]
fn test_as_str_get() {
    let method = Method(Method::Get);
    let result = method.as_str();
}

#[test]
fn test_as_str_post() {
    let method = Method(Method::Post);
    let result = method.as_str();
}

#[test]
fn test_as_str_delete() {
    let method = Method(Method::Delete);
    let result = method.as_str();
}

#[test]
fn test_as_str_head() {
    let method = Method(Method::Head);
    let result = method.as_str();
}

#[test]
fn test_as_str_options() {
    let method = Method(Method::OPTIONS);
    let result = method.as_str();
}

#[test]
fn test_as_str_trace() {
    let method = Method(Method::Trace);
    let result = method.as_str();
}

#[test]
fn test_as_str_connect() {
    let method = Method(Method::Connect);
    let result = method.as_str();
}

#[test]
fn test_as_str_patch() {
    let method = Method(Method::Patch);
    let result = method.as_str();
}

#[test]
fn test_as_str_inline_extension() {
    let inline = InlineExtension::new(b"my-inline-ext").unwrap();
    let method = Method(Method::ExtensionInline(inline));
    let result = method.as_str();
}

#[test]
fn test_as_str_allocated_extension() {
    let allocated = AllocatedExtension::new(b"my-allocated-ext").unwrap();
    let method = Method(Method::ExtensionAllocated(allocated));
    let result = method.as_str();
}

