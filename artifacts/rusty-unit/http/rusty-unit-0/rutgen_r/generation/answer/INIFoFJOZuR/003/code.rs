// Answer 0

#[derive(Debug)]
struct InvalidMethod;

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod
    }
}

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

struct AllocatedExtension(Vec<u8>);

impl AllocatedExtension {
    fn new(src: &[u8]) -> Result<Self, InvalidMethod> {
        Ok(AllocatedExtension(src.to_vec()))
    }
}

struct InlineExtension;

impl InlineExtension {
    const MAX: usize = 7; // Example max length for inline extensions
}

pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
    match src.len() {
        0 => Err(InvalidMethod::new()),
        3 => match src {
            b"GET" => Ok(Method::Get),
            b"PUT" => Ok(Method::Put),
            _ => Method::extension_inline(src),
        },
        4 => match src {
            b"POST" => Ok(Method::Post),
            b"HEAD" => Ok(Method::Head),
            _ => Method::extension_inline(src),
        },
        5 => match src {
            b"PATCH" => Ok(Method::Patch),
            b"TRACE" => Ok(Method::Trace),
            _ => Method::extension_inline(src),
        },
        6 => match src {
            b"DELETE" => Ok(Method::Delete),
            _ => Method::extension_inline(src),
        },
        7 => match src {
            b"OPTIONS" => Ok(Method::Options),
            b"CONNECT" => Ok(Method::Connect),
            _ => Method::extension_inline(src),
        },
        _ => {
            if src.len() <= InlineExtension::MAX {
                Method::extension_inline(src)
            } else {
                let allocated = AllocatedExtension::new(src)?;

                Ok(Method::ExtensionAllocated(allocated))
            }
        }
    }
}

#[test]
fn test_from_bytes_zero_length() {
    let result = from_bytes(&[]);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_three_length() {
    assert_eq!(from_bytes(b"GET"), Ok(Method::Get));
    assert_eq!(from_bytes(b"PUT"), Ok(Method::Put));
}

#[test]
fn test_from_bytes_four_length() {
    assert_eq!(from_bytes(b"POST"), Ok(Method::Post));
    assert_eq!(from_bytes(b"HEAD"), Ok(Method::Head));
}

#[test]
fn test_from_bytes_five_length() {
    assert_eq!(from_bytes(b"PATCH"), Ok(Method::Patch));
    assert_eq!(from_bytes(b"TRACE"), Ok(Method::Trace));
}

#[test]
fn test_from_bytes_six_length() {
    assert_eq!(from_bytes(b"DELETE"), Ok(Method::Delete));
}

#[test]
fn test_from_bytes_seven_length() {
    assert_eq!(from_bytes(b"OPTIONS"), Ok(Method::Options));
    assert_eq!(from_bytes(b"CONNECT"), Ok(Method::Connect));
}

#[test]
fn test_from_bytes_extension_allocated() {
    let long_input = b"EXTRA_METHOD_TOO_LONG";
    let result = from_bytes(long_input);
    assert!(result.is_ok());
    if let Ok(Method::ExtensionAllocated(allocated)) = result {
        assert_eq!(allocated.0, long_input.to_vec());
    }
}

