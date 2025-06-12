// Answer 0

#[test]
fn test_port_with_valid_uri() {
    struct TestAuthority {
        data: ByteStr,
    }

    impl Authority {
        fn port(&self) -> Option<Port<&str>> {
            if self.data.ends_with(b":80") {
                Some(Port { port: 80, repr: "" })
            } else {
                None
            }
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: TestAuthority { data: ByteStr::from_static(b"example.org:80") },
        path_and_query: PathAndQuery { data: ByteStr::new(), query: 0 },
    };

    let port = uri.port().unwrap();
    assert_eq!(port.port, 80);
}

#[test]
fn test_port_without_port_in_uri() {
    struct TestAuthority {
        data: ByteStr,
    }

    impl Authority {
        fn port(&self) -> Option<Port<&str>> {
            None
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: TestAuthority { data: ByteStr::from_static(b"example.org") },
        path_and_query: PathAndQuery { data: ByteStr::new(), query: 0 },
    };

    assert!(uri.port().is_none());
}

#[test]
fn test_port_with_relative_uri() {
    struct TestAuthority {
        data: ByteStr,
    }

    impl Authority {
        fn port(&self) -> Option<Port<&str>> {
            None
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: TestAuthority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/hello/world"), query: 0 },
    };

    assert!(uri.port().is_none());
}

#[test]
#[should_panic]
fn test_static_uri_with_invalid_port() {
    Uri::from_static("http://example.org:invalid").port();
}

