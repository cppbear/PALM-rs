// Answer 0

#[derive(Debug)]
struct HttpMethod(u8);

impl HttpMethod {
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
fn test_as_str_for_options() {
    let method = HttpMethod(HttpMethod::OPTIONS);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_for_get() {
    let method = HttpMethod(HttpMethod::GET);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_for_post() {
    let method = HttpMethod(HttpMethod::POST);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_for_put() {
    let method = HttpMethod(HttpMethod::PUT);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_for_delete() {
    let method = HttpMethod(HttpMethod::DELETE);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_for_head() {
    let method = HttpMethod(HttpMethod::HEAD);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_for_trace() {
    let method = HttpMethod(HttpMethod::TRACE);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_for_connect() {
    let method = HttpMethod(HttpMethod::CONNECT);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_for_patch() {
    let method = HttpMethod(HttpMethod::PATCH);
    assert_eq!(method.as_str(), "PATCH");
}

