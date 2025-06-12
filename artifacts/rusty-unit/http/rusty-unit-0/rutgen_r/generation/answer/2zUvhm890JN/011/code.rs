// Answer 0

#[derive(Debug)]
struct Method(Methods);

#[derive(Debug)]
enum Methods {
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
    ExtensionAllocated(String),
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self.0 {
            Methods::Options => "OPTIONS",
            Methods::Get => "GET",
            Methods::Post => "POST",
            Methods::Put => "PUT",
            Methods::Delete => "DELETE",
            Methods::Head => "HEAD",
            Methods::Trace => "TRACE",
            Methods::Connect => "CONNECT",
            Methods::Patch => "PATCH",
            Methods::ExtensionInline(ref inline) => inline.as_str(),
            Methods::ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}

#[test]
fn test_as_str_options() {
    let method = Method(Methods::Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = Method(Methods::Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = Method(Methods::Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = Method(Methods::Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = Method(Methods::Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = Method(Methods::Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = Method(Methods::Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = Method(Methods::Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = Method(Methods::Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    let method = Method(Methods::ExtensionInline("CUSTOM".to_string()));
    assert_eq!(method.as_str(), "CUSTOM");
}

#[test]
fn test_as_str_extension_allocated() {
    let method = Method(Methods::ExtensionAllocated("ALLOCATED".to_string()));
    assert_eq!(method.as_str(), "ALLOCATED");
}

