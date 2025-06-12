// Answer 0

#[derive(Debug)]
struct Response {
    status_code: u16,
    version: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    fn status(&self) -> u16 {
        self.status_code
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

impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status())
            .field("version", &self.version())
            .field("headers", self.headers())
            .field("body", self.body())
            .finish()
    }
}

#[test]
fn test_response_debug_struct() {
    let response = Response {
        status_code: 200,
        version: "HTTP/1.1".to_string(),
        headers: vec![("Content-Type".to_string(), "text/html".to_string())],
        body: "<html></html>".to_string(),
    };

    let output = format!("{:?}", response);
    assert!(output.contains("Response"));
    assert!(output.contains("status: 200"));
    assert!(output.contains("version: \"HTTP/1.1\""));
    assert!(output.contains("headers: [(\"Content-Type\", \"text/html\")]"));
    assert!(output.contains("body: \"<html></html>\""));
}

#[test]
fn test_response_debug_struct_empty() {
    let response = Response {
        status_code: 404,
        version: "HTTP/1.1".to_string(),
        headers: Vec::new(),
        body: "".to_string(),
    };

    let output = format!("{:?}", response);
    assert!(output.contains("Response"));
    assert!(output.contains("status: 404"));
    assert!(output.contains("version: \"HTTP/1.1\""));
    assert!(output.contains("headers: []"));
    assert!(output.contains("body: \"\""));
}

