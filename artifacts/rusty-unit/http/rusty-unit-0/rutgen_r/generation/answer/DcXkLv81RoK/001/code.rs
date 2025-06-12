// Answer 0

#[test]
fn test_port_with_valid_port() {
    struct Authority {
        value: &'static str,
    }

    impl Authority {
        fn port(&self) -> Option<u16> {
            let bytes = self.value;
            bytes
                .rfind(':')
                .and_then(|i| bytes[i + 1..].parse::<u16>().ok())
        }
    }

    let authority = Authority { value: "example.org:80" };
    let port = authority.port();
    assert_eq!(port, Some(80));
}

#[test]
fn test_port_without_port() {
    struct Authority {
        value: &'static str,
    }

    impl Authority {
        fn port(&self) -> Option<u16> {
            let bytes = self.value;
            bytes
                .rfind(':')
                .and_then(|i| bytes[i + 1..].parse::<u16>().ok())
        }
    }

    let authority = Authority { value: "example.org" };
    let port = authority.port();
    assert_eq!(port, None);
}

#[test]
fn test_port_with_invalid_port() {
    struct Authority {
        value: &'static str,
    }

    impl Authority {
        fn port(&self) -> Option<u16> {
            let bytes = self.value;
            bytes
                .rfind(':')
                .and_then(|i| bytes[i + 1..].parse::<u16>().ok())
        }
    }

    let authority = Authority { value: "example.org:notaport" };
    let port = authority.port();
    assert_eq!(port, None);
}

#[test]
fn test_port_with_zero() {
    struct Authority {
        value: &'static str,
    }

    impl Authority {
        fn port(&self) -> Option<u16> {
            let bytes = self.value;
            bytes
                .rfind(':')
                .and_then(|i| bytes[i + 1..].parse::<u16>().ok())
        }
    }

    let authority = Authority { value: "example.org:0" };
    let port = authority.port();
    assert_eq!(port, Some(0));
}

#[test]
fn test_port_with_large_number() {
    struct Authority {
        value: &'static str,
    }

    impl Authority {
        fn port(&self) -> Option<u16> {
            let bytes = self.value;
            bytes
                .rfind(':')
                .and_then(|i| bytes[i + 1..].parse::<u16>().ok())
        }
    }

    let authority = Authority { value: "example.org:65535" };
    let port = authority.port();
    assert_eq!(port, Some(65535));
}

