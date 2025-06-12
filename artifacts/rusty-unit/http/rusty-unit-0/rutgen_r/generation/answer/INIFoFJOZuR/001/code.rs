// Answer 0

#[derive(Debug)]
struct InvalidMethod;

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Put,
    Post,
    Head,
    Patch,
    Trace,
    Delete,
    Options,
    Connect,
    ExtensionAllocated(AllocatedExtension),
}

struct AllocatedExtension<'a> {
    data: &'a [u8],
}

impl<'a> AllocatedExtension<'a> {
    fn new(data: &'a [u8]) -> Result<Self, InvalidMethod> {
        Ok(AllocatedExtension { data })
    }
}

struct InlineExtension;

impl InlineExtension {
    const MAX: usize = 8;

    fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
        Ok(Method::ExtensionAllocated(AllocatedExtension::new(src)?))
    }
}

#[test]
fn test_from_bytes_empty() {
    let result = from_bytes(b"");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidMethod::new());
}

#[test]
fn test_from_bytes_get() {
    let result = from_bytes(b"GET");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Get);
}

#[test]
fn test_from_bytes_post() {
    let result = from_bytes(b"POST");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Post);
}

#[test]
fn test_from_bytes_patch() {
    let result = from_bytes(b"PATCH");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Patch);
}

#[test]
fn test_from_bytes_delete() {
    let result = from_bytes(b"DELETE");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Delete);
}

#[test]
fn test_from_bytes_options() {
    let result = from_bytes(b"OPTIONS");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Options);
}

#[test]
fn test_from_bytes_connect() {
    let result = from_bytes(b"CONNECT");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::Connect);
}

#[test]
fn test_from_bytes_invalid_method() {
    let result = from_bytes(b"INVALID");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_above_inline_limit() {
    let long_method = b"LONG_METHOD";
    let result = from_bytes(long_method);
    assert!(result.is_ok());
}

