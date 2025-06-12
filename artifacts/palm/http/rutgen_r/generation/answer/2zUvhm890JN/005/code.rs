// Answer 0

#[derive(Debug)]
struct HttpMethod(u8);

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self.0 {
            0 => "OPTIONS",
            1 => "GET",
            2 => "POST",
            3 => "PUT",
            4 => "DELETE",
            5 => "HEAD",
            6 => "TRACE",
            7 => "CONNECT",
            8 => "PATCH",
            _ => panic!("Unknown HTTP method"),
        }
    }
}

#[test]
fn test_options_method() {
    let method = HttpMethod(0);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_get_method() {
    let method = HttpMethod(1);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_post_method() {
    let method = HttpMethod(2);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_put_method() {
    let method = HttpMethod(3);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_delete_method() {
    let method = HttpMethod(4);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_head_method() {
    let method = HttpMethod(5);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_trace_method() {
    let method = HttpMethod(6);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_connect_method() {
    let method = HttpMethod(7);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_patch_method() {
    let method = HttpMethod(8);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
#[should_panic(expected = "Unknown HTTP method")]
fn test_invalid_method() {
    let method = HttpMethod(9);
    let _ = method.as_str();
}

