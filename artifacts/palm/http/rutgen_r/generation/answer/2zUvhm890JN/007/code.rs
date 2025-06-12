// Answer 0

#[derive(Debug)]
struct HttpMethod(MethodType);

#[derive(Debug)]
enum MethodType {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline(String),
    ExtensionAllocated(Box<String>),
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self.0 {
            MethodType::Options => "OPTIONS",
            MethodType::Get => "GET",
            MethodType::Post => "POST",
            MethodType::Put => "PUT",
            MethodType::Delete => "DELETE",
            MethodType::Head => "HEAD",
            MethodType::Trace => "TRACE",
            MethodType::Connect => "CONNECT",
            MethodType::Patch => "PATCH",
            MethodType::ExtensionInline(ref inline) => inline.as_str(),
            MethodType::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_as_str_options() {
    let method = HttpMethod(MethodType::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = HttpMethod(MethodType::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = HttpMethod(MethodType::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = HttpMethod(MethodType::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = HttpMethod(MethodType::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = HttpMethod(MethodType::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = HttpMethod(MethodType::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = HttpMethod(MethodType::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = HttpMethod(MethodType::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    let method = HttpMethod(MethodType::ExtensionInline("CUSTOM".to_string()));
    assert_eq!(method.as_str(), "CUSTOM");
}

#[test]
fn test_as_str_extension_allocated() {
    let method = HttpMethod(MethodType::ExtensionAllocated(Box::new("ALLOCATED".to_string())));
    assert_eq!(method.as_str(), "ALLOCATED");
}

