// Answer 0

#[test]
fn test_port_as_str() {
    struct Authority {
        repr: String,
    }

    impl Authority {
        fn port(&self) -> Option<&Port> {
            if let Some(pos) = self.repr.rfind(':') {
                let port_str = &self.repr[pos + 1..];
                Some(&Port { repr: port_str.to_string() })
            } else {
                None
            }
        }
    }

    struct Port {
        repr: String,
    }

    impl Port {
        fn as_str(&self) -> &str {
            self.repr.as_ref()
        }
    }

    let authority = Authority { repr: "example.org:80".to_string() };
    let port = authority.port().unwrap();
    assert_eq!(port.as_str(), "80");
}

#[test]
fn test_port_as_str_no_port() {
    struct Authority {
        repr: String,
    }

    impl Authority {
        fn port(&self) -> Option<&Port> {
            if let Some(pos) = self.repr.rfind(':') {
                let port_str = &self.repr[pos + 1..];
                Some(&Port { repr: port_str.to_string() })
            } else {
                None
            }
        }
    }

    struct Port {
        repr: String,
    }

    impl Port {
        fn as_str(&self) -> &str {
            self.repr.as_ref()
        }
    }

    let authority = Authority { repr: "example.org".to_string() };
    let port = authority.port();
    assert!(port.is_none());
}

