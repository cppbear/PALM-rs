// Answer 0

#[test]
fn test_fmt_with_scheme_authority_query() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&str> {
            self.scheme
        }

        fn authority(&self) -> Option<&str> {
            self.authority
        }

        fn path(&self) -> &str {
            self.path
        }

        fn query(&self) -> Option<&str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: Some("http"),
        authority: Some("example.com"),
        path: "/index.html",
        query: Some("id=10"),
    };

    let mut result = String::new();
    let fmt_result = uri.fmt(&mut std::fmt::Formatter::new());
    writeln!(result).unwrap();
    assert_eq!(fmt_result, Ok(()));
    assert_eq!(result, "http://example.com/index.html?id=10\n");
}

#[test]
fn test_fmt_without_scheme() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&str> {
            self.scheme
        }

        fn authority(&self) -> Option<&str> {
            self.authority
        }

        fn path(&self) -> &str {
            self.path
        }

        fn query(&self) -> Option<&str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: None,
        authority: Some("example.com"),
        path: "/index.html",
        query: None,
    };

    let mut result = String::new();
    let fmt_result = uri.fmt(&mut std::fmt::Formatter::new());
    writeln!(result).unwrap();
    assert_eq!(fmt_result, Ok(()));
    assert_eq!(result, "example.com/index.html\n");
}

#[test]
fn test_fmt_without_authority() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&str> {
            self.scheme
        }

        fn authority(&self) -> Option<&str> {
            self.authority
        }

        fn path(&self) -> &str {
            self.path
        }

        fn query(&self) -> Option<&str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: Some("http"),
        authority: None,
        path: "/index.html",
        query: Some("id=10"),
    };

    let mut result = String::new();
    let fmt_result = uri.fmt(&mut std::fmt::Formatter::new());
    writeln!(result).unwrap();
    assert_eq!(fmt_result, Ok(()));
    assert_eq!(result, "/index.html?id=10\n");
}

#[test]
fn test_fmt_without_query() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&str> {
            self.scheme
        }

        fn authority(&self) -> Option<&str> {
            self.authority
        }

        fn path(&self) -> &str {
            self.path
        }

        fn query(&self) -> Option<&str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: Some("http"),
        authority: Some("example.com"),
        path: "/index.html",
        query: None,
    };

    let mut result = String::new();
    let fmt_result = uri.fmt(&mut std::fmt::Formatter::new());
    writeln!(result).unwrap();
    assert_eq!(fmt_result, Ok(()));
    assert_eq!(result, "http://example.com/index.html\n");
}

#[test]
fn test_fmt_without_uri_components() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&str> {
            self.scheme
        }

        fn authority(&self) -> Option<&str> {
            self.authority
        }

        fn path(&self) -> &str {
            self.path
        }

        fn query(&self) -> Option<&str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: None,
        authority: None,
        path: "/",
        query: None,
    };

    let mut result = String::new();
    let fmt_result = uri.fmt(&mut std::fmt::Formatter::new());
    writeln!(result).unwrap();
    assert_eq!(fmt_result, Ok(()));
    assert_eq!(result, "/\n");
}

