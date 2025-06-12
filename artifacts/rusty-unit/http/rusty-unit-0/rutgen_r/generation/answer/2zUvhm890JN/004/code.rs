// Answer 0

#[derive(Debug)]
struct HttpMethod(InnerHttpMethod);

#[derive(Debug)]
enum InnerHttpMethod {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline(Box<str>),
    ExtensionAllocated(Box<str>),
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self.0 {
            InnerHttpMethod::Options => "OPTIONS",
            InnerHttpMethod::Get => "GET",
            InnerHttpMethod::Post => "POST",
            InnerHttpMethod::Put => "PUT",
            InnerHttpMethod::Delete => "DELETE",
            InnerHttpMethod::Head => "HEAD",
            InnerHttpMethod::Trace => "TRACE",
            InnerHttpMethod::Connect => "CONNECT",
            InnerHttpMethod::Patch => "PATCH",
            InnerHttpMethod::ExtensionInline(ref inline) => inline.as_str(),
            InnerHttpMethod::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_as_str_connect() {
    let method = HttpMethod(InnerHttpMethod::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_options() {
    let method = HttpMethod(InnerHttpMethod::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = HttpMethod(InnerHttpMethod::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = HttpMethod(InnerHttpMethod::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = HttpMethod(InnerHttpMethod::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = HttpMethod(InnerHttpMethod::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = HttpMethod(InnerHttpMethod::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = HttpMethod(InnerHttpMethod::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_patch() {
    let method = HttpMethod(InnerHttpMethod::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    let method = HttpMethod(InnerHttpMethod::ExtensionInline(Box::from("CUSTOM")));
    assert_eq!(method.as_str(), "CUSTOM");
}

#[test]
fn test_as_str_extension_allocated() {
    let method = HttpMethod(InnerHttpMethod::ExtensionAllocated(Box::from("ALLOCATED_CUSTOM")));
    assert_eq!(method.as_str(), "ALLOCATED_CUSTOM");
}

