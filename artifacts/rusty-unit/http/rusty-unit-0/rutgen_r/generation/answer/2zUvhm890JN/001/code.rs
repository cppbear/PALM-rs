// Answer 0

#[derive(Debug)]
struct MethodWrapper(Method);

#[derive(Debug)]
enum Method {
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

impl MethodWrapper {
    pub fn as_str(&self) -> &str {
        match self.0 {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
            Method::Patch => "PATCH",
            Method::ExtensionInline(ref inline) => inline.as_str(),
            Method::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_as_str_options() {
    let method = MethodWrapper(Method::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = MethodWrapper(Method::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = MethodWrapper(Method::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = MethodWrapper(Method::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = MethodWrapper(Method::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = MethodWrapper(Method::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = MethodWrapper(Method::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = MethodWrapper(Method::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = MethodWrapper(Method::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    let method = MethodWrapper(Method::ExtensionInline(Box::from("CUSTOM_METHOD")));
    assert_eq!(method.as_str(), "CUSTOM_METHOD");
}

#[test]
fn test_as_str_extension_allocated() {
    let method = MethodWrapper(Method::ExtensionAllocated(Box::from("ALLOCATED_METHOD")));
    assert_eq!(method.as_str(), "ALLOCATED_METHOD");
}

