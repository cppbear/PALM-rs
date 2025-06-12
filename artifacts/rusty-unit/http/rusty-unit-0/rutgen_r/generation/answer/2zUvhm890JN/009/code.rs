// Answer 0

#[derive(Debug)]
struct HttpMethod(String);

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self.0.as_str() {
            "OPTIONS" => "OPTIONS",
            "GET" => "GET",
            "POST" => "POST",
            "PUT" => "PUT",
            "DELETE" => "DELETE",
            "HEAD" => "HEAD",
            "TRACE" => "TRACE",
            "CONNECT" => "CONNECT",
            "PATCH" => "PATCH",
            _ => panic!("Unknown HTTP method"),
        }
    }
}

#[test]
fn test_as_str_options() {
    let method = HttpMethod("OPTIONS".to_string());
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    let method = HttpMethod("GET".to_string());
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    let method = HttpMethod("POST".to_string());
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_put() {
    let method = HttpMethod("PUT".to_string());
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_delete() {
    let method = HttpMethod("DELETE".to_string());
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    let method = HttpMethod("HEAD".to_string());
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    let method = HttpMethod("TRACE".to_string());
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    let method = HttpMethod("CONNECT".to_string());
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    let method = HttpMethod("PATCH".to_string());
    assert_eq!(method.as_str(), "PATCH");
}

#[should_panic(expected = "Unknown HTTP method")]
#[test]
fn test_as_str_unknown() {
    let method = HttpMethod("UNKNOWN".to_string());
    method.as_str();
}

