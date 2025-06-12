// Answer 0

#[derive(Debug)]
struct Method(u8);

impl Method {
    const OPTIONS: u8 = 0;
    const GET: u8 = 1;
    const POST: u8 = 2;
    const PUT: u8 = 3;
    const DELETE: u8 = 4;
    const HEAD: u8 = 5;
    const TRACE: u8 = 6;
    const CONNECT: u8 = 7;
    const PATCH: u8 = 8;

    pub fn as_str(&self) -> &str {
        match self.0 {
            Self::OPTIONS => "OPTIONS",
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            Self::TRACE => "TRACE",
            Self::CONNECT => "CONNECT",
            Self::PATCH => "PATCH",
            _ => "UNKNOWN",
        }
    }
}

#[test]
fn test_as_str_options() {
    let method = Method(Method::OPTIONS);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = Method(Method::GET);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = Method(Method::POST);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = Method(Method::PUT);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = Method(Method::DELETE);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = Method(Method::HEAD);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = Method(Method::TRACE);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = Method(Method::CONNECT);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = Method(Method::PATCH);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_unknown() {
    let method = Method(99); // Arbitrary unknown value
    assert_eq!(method.as_str(), "UNKNOWN");
}

