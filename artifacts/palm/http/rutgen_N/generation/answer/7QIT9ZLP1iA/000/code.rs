// Answer 0

#[test]
fn test_fmt_debug_struct() {
    use std::fmt;

    struct Parts {
        status: u16,
        version: u8,
        headers: Vec<String>,
    }

    let parts = Parts {
        status: 200,
        version: 1,
        headers: vec!["Content-Type: application/json".to_string(), "Content-Length: 123".to_string()],
    };

    let mut buffer = String::new();
    let result = fmt::write(&mut buffer, |f| {
        f.debug_struct("Parts")
            .field("status", &parts.status)
            .field("version", &parts.version)
            .field("headers", &parts.headers)
            .finish()
    });

    assert!(result.is_ok());
    assert!(buffer.contains("Parts {"));
    assert!(buffer.contains("status: 200"));
    assert!(buffer.contains("version: 1"));
    assert!(buffer.contains("headers: [\"Content-Type: application/json\", \"Content-Length: 123\"]"));
}

#[test]
fn test_fmt_debug_struct_empty_headers() {
    use std::fmt;

    struct Parts {
        status: u16,
        version: u8,
        headers: Vec<String>,
    }

    let parts = Parts {
        status: 404,
        version: 1,
        headers: Vec::new(),
    };

    let mut buffer = String::new();
    let result = fmt::write(&mut buffer, |f| {
        f.debug_struct("Parts")
            .field("status", &parts.status)
            .field("version", &parts.version)
            .field("headers", &parts.headers)
            .finish()
    });

    assert!(result.is_ok());
    assert!(buffer.contains("Parts {"));
    assert!(buffer.contains("status: 404"));
    assert!(buffer.contains("version: 1"));
    assert!(buffer.contains("headers: []"));
}

