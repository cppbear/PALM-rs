// Answer 0

fn test_fmt_with_valid_data() {
    struct Uri {
        scheme: Option<String>,
        authority: Option<String>,
        path: String,
        query: Option<String>,
    }

    impl Uri {
        fn scheme(&self) -> &Option<String> {
            &self.scheme
        }

        fn authority(&self) -> &Option<String> {
            &self.authority
        }

        fn path(&self) -> &String {
            &self.path
        }

        fn query(&self) -> &Option<String> {
            &self.query
        }
    }

    let uri = Uri {
        scheme: Some("http".to_string()),
        authority: Some("www.example.com".to_string()),
        path: "/path".to_string(),
        query: Some("key=value".to_string()),
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| uri.fmt(f));

    assert!(result.is_ok());
    assert_eq!(output, "http://www.example.com/path?key=value");
}

#[test]
#[should_panic]
fn test_fmt_with_write_error_on_query() {
    struct FaultyWriteFormatter;

    impl std::fmt::Write for FaultyWriteFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Err(std::fmt::Error) // Simulate a write error
        }
    }

    struct Uri {
        scheme: Option<String>,
        authority: Option<String>,
        path: String,
        query: Option<String>,
    }

    impl Uri {
        fn scheme(&self) -> &Option<String> {
            &self.scheme
        }

        fn authority(&self) -> &Option<String> {
            &self.authority
        }

        fn path(&self) -> &String {
            &self.path
        }

        fn query(&self) -> &Option<String> {
            &self.query
        }
    }

    let uri = Uri {
        scheme: Some("http".to_string()),
        authority: Some("www.example.com".to_string()),
        path: "/path".to_string(),
        query: Some("key=value".to_string()),
    };

    let mut output = FaultyWriteFormatter;
    uri.fmt(&mut output).unwrap(); // This is expected to panic
}

