// Answer 0

#[test]
fn test_fmt_with_panic_on_path() {
    struct TestUri {
        scheme: Option<&'static str>,
        authority: Option<&'static str>,
        path: &'static str,
        query: Option<&'static str>,
    }

    impl TestUri {
        fn scheme(&self) -> Option<&'static str> {
            self.scheme
        }

        fn authority(&self) -> Option<&'static str> {
            self.authority
        }

        fn path(&self) -> &'static str {
            self.path
        }

        fn query(&self) -> Option<&'static str> {
            self.query
        }
    }

    let uri = TestUri {
        scheme: Some("http"),
        authority: Some("example.com"),
        path: "path/to/resource",
        query: Some("key=value"),
    };

    // Attempt to write to a buffer and expect it to panic
    let result = std::panic::catch_unwind(|| {
        let mut buf = Vec::new();
        let _ = write!(buf, "{}", uri);
    });

    // Verify that the result is a panic
    assert!(result.is_err());
}

