// Answer 0

#[derive(Debug)]
struct Parts {
    status: u16,
    version: String,
    headers: Vec<(String, String)>,
}

impl Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parts")
            .field("status", &self.status)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .finish()
    }
}

#[test]
fn test_fmt_with_valid_data() {
    let parts = Parts {
        status: 200,
        version: "HTTP/1.1".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), "1234".to_string()),
        ],
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", parts);

    assert!(result.is_ok());
    assert!(output.contains("Parts"));
    assert!(output.contains("status: 200"));
    assert!(output.contains("version: \"HTTP/1.1\""));
    assert!(output.contains("headers: ["));
    assert!(output.contains("Content-Type: \"application/json\""));
    assert!(output.contains("Content-Length: \"1234\""));
}

#[test]
fn test_fmt_with_empty_headers() {
    let parts = Parts {
        status: 404,
        version: "HTTP/1.0".to_string(),
        headers: vec![],
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", parts);

    assert!(result.is_ok());
    assert!(output.contains("Parts"));
    assert!(output.contains("status: 404"));
    assert!(output.contains("version: \"HTTP/1.0\""));
    assert!(output.contains("headers: []"));
}

#[test]
fn test_fmt_with_large_status() {
    let parts = Parts {
        status: 999,
        version: "HTTP/2.0".to_string(),
        headers: vec![
            ("X-Custom-Header".to_string(), "Some Value".to_string()),
        ],
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", parts);

    assert!(result.is_ok());
    assert!(output.contains("Parts"));
    assert!(output.contains("status: 999"));
    assert!(output.contains("version: \"HTTP/2.0\""));
    assert!(output.contains("headers: ["));
    assert!(output.contains("X-Custom-Header: \"Some Value\""));
}

