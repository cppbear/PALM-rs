// Answer 0

#[test]
fn test_as_str_patch() {
    let method = Method(Method::Patch);
    method.as_str();
}

#[test]
fn test_as_str_options() {
    let method = Method(Method::OPTIONS);
    method.as_str();
}

#[test]
fn test_as_str_get() {
    let method = Method(Method::GET);
    method.as_str();
}

#[test]
fn test_as_str_post() {
    let method = Method(Method::POST);
    method.as_str();
}

#[test]
fn test_as_str_put() {
    let method = Method(Method::PUT);
    method.as_str();
}

#[test]
fn test_as_str_delete() {
    let method = Method(Method::DELETE);
    method.as_str();
}

#[test]
fn test_as_str_head() {
    let method = Method(Method::HEAD);
    method.as_str();
}

#[test]
fn test_as_str_trace() {
    let method = Method(Method::TRACE);
    method.as_str();
}

#[test]
fn test_as_str_connect() {
    let method = Method(Method::CONNECT);
    method.as_str();
}

#[test]
fn test_as_str_extension_inline() {
    let inline_extension = InlineExtension::new("inline_extension".as_bytes()).unwrap();
    let method = Method(Inner::ExtensionInline(inline_extension));
    method.as_str();
}

#[test]
fn test_as_str_extension_allocated() {
    let allocated_extension = AllocatedExtension::new("allocated_extension".as_bytes()).unwrap();
    let method = Method(Inner::ExtensionAllocated(allocated_extension));
    method.as_str();
}

