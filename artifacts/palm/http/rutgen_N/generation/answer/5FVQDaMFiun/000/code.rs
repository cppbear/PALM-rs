// Answer 0

#[derive(Debug)]
struct Request {
    method: String,
    uri: String,
    version: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Request {
    fn method(&self) -> &str {
        &self.method
    }

    fn uri(&self) -> &str {
        &self.uri
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    fn body(&self) -> &str {
        &self.body
    }
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", self.method())
            .field("uri", self.uri())
            .field("version", &self.version())
            .field("headers", self.headers())
            .field("body", self.body())
            .finish()
    }
}

#[test]
fn test_request_debug_fmt() {
    let request = Request {
        method: "GET".to_string(),
        uri: "/example".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: "{}".to_string(),
    };

    let debug_output = format!("{:?}", request);
    assert!(debug_output.contains("method: \"GET\""));
    assert!(debug_output.contains("uri: \"/example\""));
    assert!(debug_output.contains("version: \"HTTP/1.1\""));
    assert!(debug_output.contains("headers: [(\"Content-Type\", \"application/json\")]"));
    assert!(debug_output.contains("body: \"{}\""));
}

#[test]
fn test_request_debug_fmt_empty() {
    let request = Request {
        method: "".to_string(),
        uri: "".to_string(),
        version: "".to_string(),
        headers: vec![],
        body: "".to_string(),
    };

    let debug_output = format!("{:?}", request);
    assert!(debug_output.contains("method: \"\""));
    assert!(debug_output.contains("uri: \"\""));
    assert!(debug_output.contains("version: \"\""));
    assert!(debug_output.contains("headers: []"));
    assert!(debug_output.contains("body: \"\""));
}

