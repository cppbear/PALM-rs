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

#[test]
fn test_port_u16_with_high_value() {
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

    let authority = Authority { port: Some(65535) };
    assert_eq!(authority.port_u16(), Some(65535));
}

#[test]
fn test_port_u16_with_invalid_high_value() {
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

    let authority = Authority { port: Some(70000) }; // Simulating out of bound as u16
    assert_eq!(authority.port_u16(), Some(70000)); // Note: This case should not happen in a real scenario, served for conceptual coverage.
}

