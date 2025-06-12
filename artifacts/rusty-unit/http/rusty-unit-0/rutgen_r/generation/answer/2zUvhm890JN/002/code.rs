// Answer 0

#[derive(Debug)]
struct ExtensionInline(String);

impl ExtensionInline {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
struct ExtensionAllocated(String);

impl ExtensionAllocated {
    fn as_str(&self) -> &str {
        &self.0
    }
}

struct HttpMethod(InnerMethod);

enum InnerMethod {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline(ExtensionInline),
    ExtensionAllocated(ExtensionAllocated),
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self.0 {
            InnerMethod::Options => "OPTIONS",
            InnerMethod::Get => "GET",
            InnerMethod::Post => "POST",
            InnerMethod::Put => "PUT",
            InnerMethod::Delete => "DELETE",
            InnerMethod::Head => "HEAD",
            InnerMethod::Trace => "TRACE",
            InnerMethod::Connect => "CONNECT",
            InnerMethod::Patch => "PATCH",
            InnerMethod::ExtensionInline(ref inline) => inline.as_str(),
            InnerMethod::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_http_method_options() {
    let method = HttpMethod(InnerMethod::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_http_method_get() {
    let method = HttpMethod(InnerMethod::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_http_method_post() {
    let method = HttpMethod(InnerMethod::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_http_method_put() {
    let method = HttpMethod(InnerMethod::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_http_method_delete() {
    let method = HttpMethod(InnerMethod::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_http_method_head() {
    let method = HttpMethod(InnerMethod::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_http_method_trace() {
    let method = HttpMethod(InnerMethod::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_http_method_connect() {
    let method = HttpMethod(InnerMethod::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_http_method_patch() {
    let method = HttpMethod(InnerMethod::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_http_method_extension_inline() {
    let inline_method = ExtensionInline("CUSTOM_INLINE".to_string());
    let method = HttpMethod(InnerMethod::ExtensionInline(inline_method));
    assert_eq!(method.as_str(), "CUSTOM_INLINE");
}

#[test]
fn test_http_method_extension_allocated() {
    let allocated_method = ExtensionAllocated("CUSTOM_ALLOCATED".to_string());
    let method = HttpMethod(InnerMethod::ExtensionAllocated(allocated_method));
    assert_eq!(method.as_str(), "CUSTOM_ALLOCATED");
}

