// Answer 0

#[derive(Debug)]
struct TestUri {
    scheme: Option<String>,
    authority: Option<String>,
    path: String,
    query: Option<String>,
}

impl TestUri {
    fn scheme(&self) -> Option<&String> {
        self.scheme.as_ref()
    }

    fn authority(&self) -> Option<&String> {
        self.authority.as_ref()
    }

    fn path(&self) -> &String {
        &self.path
    }

    fn query(&self) -> Option<&String> {
        self.query.as_ref()
    }
}

#[test]
fn test_uri_with_all_parts() {
    let uri = TestUri {
        scheme: Some("http".to_string()),
        authority: Some("www.example.com".to_string()),
        path: "/path/to/resource".to_string(),
        query: Some("key=value".to_string()),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", uri);
    assert!(result.is_ok());
    assert_eq!(output, "http://www.example.com/path/to/resource?key=value");
}

#[test]
fn test_uri_without_query() {
    let uri = TestUri {
        scheme: Some("https".to_string()),
        authority: Some("www.example.com".to_string()),
        path: "/path/to/resource".to_string(),
        query: None,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", uri);
    assert!(result.is_ok());
    assert_eq!(output, "https://www.example.com/path/to/resource");
}

#[test]
fn test_uri_without_authority() {
    let uri = TestUri {
        scheme: Some("ftp".to_string()),
        authority: None,
        path: "/path/to/resource".to_string(),
        query: Some("key=value".to_string()),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", uri);
    assert!(result.is_ok());
    assert_eq!(output, "ftp:///path/to/resource?key=value");
}

