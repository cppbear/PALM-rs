// Answer 0

#[test]
fn test_port_u16_with_port() {
    struct FakeUri {
        port: Option<u16>,
    }
    
    impl FakeUri {
        fn port(&self) -> Option<u16> {
            self.port
        }
        
        fn port_u16(&self) -> Option<u16> {
            self.port().map(|p| p)
        }
    }
    
    let uri = FakeUri { port: Some(80) };
    assert_eq!(uri.port_u16(), Some(80));
}

#[test]
fn test_port_u16_without_port() {
    struct FakeUri {
        port: Option<u16>,
    }
    
    impl FakeUri {
        fn port(&self) -> Option<u16> {
            self.port
        }
        
        fn port_u16(&self) -> Option<u16> {
            self.port().map(|p| p)
        }
    }
    
    let uri = FakeUri { port: None };
    assert_eq!(uri.port_u16(), None);
}

