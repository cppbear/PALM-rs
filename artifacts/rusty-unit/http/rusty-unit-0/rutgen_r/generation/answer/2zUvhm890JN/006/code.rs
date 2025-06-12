// Answer 0

#[derive(Debug)]
struct HttpMethod(HttpMethodType);

#[derive(Debug)]
enum HttpMethodType {
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
            HttpMethodType::Options => "OPTIONS",
            HttpMethodType::Get => "GET",
            HttpMethodType::Post => "POST",
            HttpMethodType::Put => "PUT",
            HttpMethodType::Delete => "DELETE",
            HttpMethodType::Head => "HEAD",
            HttpMethodType::Trace => "TRACE",
            HttpMethodType::Connect => "CONNECT",
            HttpMethodType::Patch => "PATCH",
            HttpMethodType::ExtensionInline(ref inline) => inline.as_str(),
            HttpMethodType::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_http_method_as_str_options() {
    let method = HttpMethod(HttpMethodType::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_http_method_as_str_get() {
    let method = HttpMethod(HttpMethodType::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_http_method_as_str_post() {
    let method = HttpMethod(HttpMethodType::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_http_method_as_str_put() {
    let method = HttpMethod(HttpMethodType::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_http_method_as_str_delete() {
    let method = HttpMethod(HttpMethodType::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_http_method_as_str_head() {
    let method = HttpMethod(HttpMethodType::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_http_method_as_str_trace() {
    let method = HttpMethod(HttpMethodType::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_http_method_as_str_connect() {
    let method = HttpMethod(HttpMethodType::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_http_method_as_str_patch() {
    let method = HttpMethod(HttpMethodType::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_http_method_as_str_extension_inline() {
    let method = HttpMethod(HttpMethodType::ExtensionInline("CUSTOM".to_string()));
    assert_eq!(method.as_str(), "CUSTOM");
}

#[test]
fn test_http_method_as_str_extension_allocated() {
    let method = HttpMethod(HttpMethodType::ExtensionAllocated(Box::new("DYNAMIC".to_string())));
    assert_eq!(method.as_str(), "DYNAMIC");
}

