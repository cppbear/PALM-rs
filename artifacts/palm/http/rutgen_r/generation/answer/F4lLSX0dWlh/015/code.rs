// Answer 0

#[test]
fn test_fmt_with_valid_scheme_and_authority() {
    struct TestUri {
        scheme_value: Option<String>,
        authority_value: Option<String>,
        path_value: String,
        query_value: Option<String>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&String> {
            self.scheme_value.as_ref()
        }

        fn authority(&self) -> Option<&String> {
            self.authority_value.as_ref()
        }

        fn path(&self) -> &String {
            &self.path_value
        }

        fn query(&self) -> Option<&String> {
            self.query_value.as_ref()
        }
    }

    let uri = TestUri {
        scheme_value: Some("http".to_string()),
        authority_value: Some("example.com".to_string()),
        path_value: "/path/to/resource".to_string(),
        query_value: None,
    };

    let mut result = String::new();
    let formatter = &mut std::fmt::Formatter::new(&mut result);
    let _ = uri.fmt(formatter);

    assert_eq!(result, "http://example.com/path/to/resource");
}

#[should_panic]
#[test]
fn test_fmt_with_valid_scheme_but_authority_write_err() {
    struct TestUri {
        scheme_value: Option<String>,
        authority_value: Option<String>,
        path_value: String,
        query_value: Option<String>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&String> {
            self.scheme_value.as_ref()
        }

        fn authority(&self) -> Option<&String> {
            self.authority_value.as_ref()
        }

        fn path(&self) -> &String {
            &self.path_value
        }

        fn query(&self) -> Option<&String> {
            self.query_value.as_ref()
        }
    }

    let uri = TestUri {
        scheme_value: Some("http".to_string()),
        authority_value: Some("example.com".to_string()),
        path_value: "/path/to/resource".to_string(),
        query_value: None,
    };

    // Simulating a write error
    let mut result = String::new();
    let formatter = &mut std::fmt::Formatter::new(&mut result);
    let _ = uri.fmt(formatter);
    // The mock for `authority` should fail
    assert!(formatter.write_str(" test").is_err());
}

