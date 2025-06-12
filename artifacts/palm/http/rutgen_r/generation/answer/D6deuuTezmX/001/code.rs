// Answer 0

#[test]
fn test_port_as_u16_valid() {
    struct Authority {
        port: u16,
    }

    impl Authority {
        pub fn port(&self) -> Option<&Authority> {
            Some(self)
        }

        pub const fn as_u16(&self) -> u16 {
            self.port
        }
    }

    let authority = Authority { port: 80 };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_port_as_u16_zero() {
    struct Authority {
        port: u16,
    }

    impl Authority {
        pub fn port(&self) -> Option<&Authority> {
            Some(self)
        }

        pub const fn as_u16(&self) -> u16 {
            self.port
        }
    }

    let authority = Authority { port: 0 };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 0);
}

#[test]
fn test_port_as_u16_max() {
    struct Authority {
        port: u16,
    }

    impl Authority {
        pub fn port(&self) -> Option<&Authority> {
            Some(self)
        }

        pub const fn as_u16(&self) -> u16 {
            self.port
        }
    }

    let authority = Authority { port: u16::MAX };
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), u16::MAX);
}

