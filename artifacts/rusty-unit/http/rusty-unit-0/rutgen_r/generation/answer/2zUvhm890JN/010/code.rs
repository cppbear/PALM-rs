// Answer 0

#[derive(Debug)]
struct HttpMethodWrapper(HttpMethod);

#[derive(Debug)]
enum HttpMethod {
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

impl HttpMethodWrapper {
    pub fn as_str(&self) -> &str {
        match self.0 {
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::ExtensionInline(ref inline) => inline.as_str(),
            HttpMethod::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_http_method_as_str_options() {
    let method = HttpMethodWrapper(HttpMethod::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_http_method_as_str_get() {
    let method = HttpMethodWrapper(HttpMethod::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_http_method_as_str_post() {
    let method = HttpMethodWrapper(HttpMethod::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_http_method_as_str_put() {
    let method = HttpMethodWrapper(HttpMethod::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_http_method_as_str_delete() {
    let method = HttpMethodWrapper(HttpMethod::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_http_method_as_str_head() {
    let method = HttpMethodWrapper(HttpMethod::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_http_method_as_str_trace() {
    let method = HttpMethodWrapper(HttpMethod::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_http_method_as_str_connect() {
    let method = HttpMethodWrapper(HttpMethod::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_http_method_as_str_patch() {
    let method = HttpMethodWrapper(HttpMethod::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_http_method_as_str_extension_inline() {
    let method = HttpMethodWrapper(HttpMethod::ExtensionInline("CUSTOM".to_string()));
    assert_eq!(method.as_str(), "CUSTOM");
}

#[test]
fn test_http_method_as_str_extension_allocated() {
    let method = HttpMethodWrapper(HttpMethod::ExtensionAllocated(Box::new("ALLOTTED".to_string())));
    assert_eq!(method.as_str(), "ALLOTTED");
}

