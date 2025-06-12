// Answer 0

#[test]
fn test_fmt_with_valid_data() {
    use std::fmt;

    struct Parts {
        method: String,
        uri: String,
        version: String,
        headers: Vec<(String, String)>,
    }

    impl fmt::Debug for Parts {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Parts")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("version", &self.version)
                .field("headers", &self.headers)
                .finish()
        }
    }

    let parts = Parts {
        method: "GET".to_string(),
        uri: "http://example.com".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("User-Agent".to_string(), "test-agent".to_string()),
        ],
    };

    let result = format!("{:?}", parts);
    assert!(result.contains("method: \"GET\""));
    assert!(result.contains("uri: \"http://example.com\""));
    assert!(result.contains("version: \"HTTP/1.1\""));
    assert!(result.contains("headers: [("));
}

#[test]
fn test_fmt_with_empty_fields() {
    use std::fmt;

    struct Parts {
        method: String,
        uri: String,
        version: String,
        headers: Vec<(String, String)>,
    }

    impl fmt::Debug for Parts {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Parts")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("version", &self.version)
                .field("headers", &self.headers)
                .finish()
        }
    }

    let parts = Parts {
        method: "".to_string(),
        uri: "".to_string(),
        version: "".to_string(),
        headers: vec![],
    };

    let result = format!("{:?}", parts);
    assert!(result.contains("method: \"\""));
    assert!(result.contains("uri: \"\""));
    assert!(result.contains("version: \"\""));
    assert!(result.contains("headers: []"));
}

#[test]
fn test_fmt_with_large_headers() {
    use std::fmt;

    struct Parts {
        method: String,
        uri: String,
        version: String,
        headers: Vec<(String, String)>,
    }

    impl fmt::Debug for Parts {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Parts")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("version", &self.version)
                .field("headers", &self.headers)
                .finish()
        }
    }

    let large_header_key = "X-Long-Header-Name".repeat(20);
    let large_header_value = "Value".repeat(100);

    let parts = Parts {
        method: "POST".to_string(),
        uri: "http://example.com".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![(large_header_key.clone(), large_header_value.clone())],
    };

    let result = format!("{:?}", parts);
    assert!(result.contains(&format!("method: \"POST\"")));
    assert!(result.contains(&format!("uri: \"http://example.com\"")));
    assert!(result.contains(&format!("version: \"HTTP/1.1\"")));
    assert!(result.contains(&format!("headers: [(\"{}\", \"{}\")]",
        large_header_key, large_header_value)));
}

