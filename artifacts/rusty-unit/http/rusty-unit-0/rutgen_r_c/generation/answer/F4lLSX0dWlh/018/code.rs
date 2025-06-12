// Answer 0

#[test]
fn test_uri_display_with_scheme_and_authority_and_query_should_panic() {
    struct MockScheme;
    
    impl fmt::Display for MockScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "http")
        }
    }
    
    struct MockAuthority;
    
    impl fmt::Display for MockAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "example.com")
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::new() },
        authority: Authority { data: ByteStr::from_static(b"example.com") },
        path_and_query: PathAndQuery { 
            data: ByteStr::from_static(b"/path"), 
            query: 123 
        },
    };
    
    let result = std::panic::catch_unwind(|| {
        let _ = format!("{}", uri);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_uri_display_with_scheme_and_authority_without_query() {
    struct MockScheme;
    
    impl fmt::Display for MockScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "https")
        }
    }
    
    struct MockAuthority;
    
    impl fmt::Display for MockAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "example.com")
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::new() },
        authority: Authority { data: ByteStr::from_static(b"example.com") },
        path_and_query: PathAndQuery { 
            data: ByteStr::from_static(b"/path"), 
            query: 0 
        },
    };
    
    let formatted = format!("{}", uri);
    assert_eq!(formatted, "https://example.com/path");
}

#[test]
fn test_uri_display_with_scheme_without_authority_and_query() {
    struct MockScheme;
    
    impl fmt::Display for MockScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ftp")
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::new() },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { 
            data: ByteStr::from_static(b"/path"), 
            query: 0 
        },
    };
    
    let formatted = format!("{}", uri);
    assert_eq!(formatted, "ftp:///path");
}

