// Answer 0

#[derive(Debug)]
struct TestUri {
    scheme_value: Option<&'static str>,
    authority_value: Option<&'static str>,
    path_value: &'static str,
    query_value: Option<&'static str>,
}

impl TestUri {
    fn scheme(&self) -> Option<&str> {
        self.scheme_value
    }

    fn authority(&self) -> Option<&str> {
        self.authority_value
    }

    fn path(&self) -> &str {
        self.path_value
    }

    fn query(&self) -> Option<&str> {
        self.query_value
    }
}

#[test]
fn test_fmt_with_valid_scheme_authority_and_path() {
    let uri = TestUri {
        scheme_value: Some("http"),
        authority_value: Some("www.example.com"),
        path_value: "/path/to/resource",
        query_value: Some("key=value"),
    };
    
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| uri.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "http://www.example.com/path/to/resource?key=value");
}

#[test]
fn test_fmt_with_missing_authority() {
    let uri = TestUri {
        scheme_value: Some("https"),
        authority_value: None,
        path_value: "/path/to/resource",
        query_value: Some("key=value"),
    };
    
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| uri.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "https:///path/to/resource?key=value");
}

#[test]
fn test_fmt_with_missing_query() {
    let uri = TestUri {
        scheme_value: Some("ftp"),
        authority_value: Some("ftp.example.com"),
        path_value: "/path/to/resource",
        query_value: None,
    };
    
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| uri.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "ftp://ftp.example.com/path/to/resource");
}

#[should_panic]
#[test]
fn test_fmt_with_err_on_write() {
    let uri = TestUri {
        scheme_value: Some("http"),
        authority_value: Some("www.example.com"),
        path_value: "/", // This path could create issues if it results in Err
        query_value: None,
    };
    
    // Duplicating the formatting should create an Err condition.
    let mut output = String::new();
    write!(output, "Some incorrect formatting").unwrap(); // Precedent write to cause panic.
    let result = std::fmt::write(&mut output, |f| uri.fmt(f));
    
    assert!(result.is_err()); // Expect the method to hit the Err case.
}

