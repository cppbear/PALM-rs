// Answer 0

#[derive(Debug)]
struct Authority {
    port: u16,
}

impl Authority {
    fn port(&self) -> Option<&Self> {
        Some(self)
    }
}

#[test]
fn test_port() {
    let authority = Authority { port: 80 };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_port_boundary() {
    let authority = Authority { port: 0 };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 0);
}

#[test]
fn test_large_port() {
    let authority = Authority { port: 65535 };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 65535);
}

