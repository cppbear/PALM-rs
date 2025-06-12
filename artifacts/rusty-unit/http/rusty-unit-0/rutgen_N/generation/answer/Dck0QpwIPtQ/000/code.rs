// Answer 0

#[derive(Debug)]
struct StatusCode(u16);

impl StatusCode {
    pub fn canonical_reason(&self) -> Option<&'static str> {
        match self.0 {
            200 => Some("OK"),
            404 => Some("Not Found"),
            500 => Some("Internal Server Error"),
            _ => None,
        }
    }
}

#[test]
fn test_canonical_reason_ok() {
    let status = StatusCode(200);
    assert_eq!(status.canonical_reason(), Some("OK"));
}

#[test]
fn test_canonical_reason_not_found() {
    let status = StatusCode(404);
    assert_eq!(status.canonical_reason(), Some("Not Found"));
}

#[test]
fn test_canonical_reason_internal_server_error() {
    let status = StatusCode(500);
    assert_eq!(status.canonical_reason(), Some("Internal Server Error"));
}

#[test]
fn test_canonical_reason_unknown() {
    let status = StatusCode(418);
    assert_eq!(status.canonical_reason(), None);
}

