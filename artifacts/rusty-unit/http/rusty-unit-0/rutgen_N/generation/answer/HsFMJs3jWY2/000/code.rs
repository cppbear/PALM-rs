// Answer 0

#[derive(Debug)]
struct RequestParts {
    method: String,
    uri: String,
    version: String,
    headers: Vec<(String, String)>,
}

impl std::fmt::Debug for RequestParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parts")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .finish()
    }
}

#[test]
fn test_request_parts_debug_fmt() {
    let parts = RequestParts {
        method: "GET".to_string(),
        uri: "/example".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
    };
    
    let expected = format!(
        "Parts {{ method: \"{}\", uri: \"{}\", version: \"{}\", headers: {:?} }}",
        parts.method, parts.uri, parts.version, parts.headers
    );
    
    assert_eq!(format!("{:?}", parts), expected);
}

#[test]
fn test_request_parts_empty_headers() {
    let parts = RequestParts {
        method: "POST".to_string(),
        uri: "/submit".to_string(),
        version: "HTTP/1.0".to_string(),
        headers: Vec::new(),
    };
    
    let expected = "Parts { method: \"POST\", uri: \"/submit\", version: \"HTTP/1.0\", headers: [] }";
    
    assert_eq!(format!("{:?}", parts), expected);
}

#[test]
fn test_request_parts_with_multiple_headers() {
    let parts = RequestParts {
        method: "PUT".to_string(),
        uri: "/update".to_string(),
        version: "HTTP/2.0".to_string(),
        headers: vec![
            ("Authorization".to_string(), "Bearer token".to_string()),
            ("Accept".to_string(), "application/xml".to_string())
        ],
    };
    
    let expected = format!(
        "Parts {{ method: \"{}\", uri: \"{}\", version: \"{}\", headers: {:?} }}",
        parts.method, parts.uri, parts.version, parts.headers
    );
    
    assert_eq!(format!("{:?}", parts), expected);
}

