// Answer 0

#[test]
fn test_port_u16_with_valid_port() {
    struct Authority {
        port: Option<u16>,
    }
    
    impl Authority {
        fn port(&self) -> Option<u16> {
            self.port
        }

        fn port_u16(&self) -> Option<u16> {
            self.port().map(|p| p)
        }
    }

    let authority = Authority { port: Some(80) };
    assert_eq!(authority.port_u16(), Some(80));
}

#[test]
fn test_port_u16_with_no_port() {
    struct Authority {
        port: Option<u16>,
    }
    
    impl Authority {
        fn port(&self) -> Option<u16> {
            self.port
        }

        fn port_u16(&self) -> Option<u16> {
            self.port().map(|p| p)
        }
    }

    let authority = Authority { port: None };
    assert_eq!(authority.port_u16(), None);
}

