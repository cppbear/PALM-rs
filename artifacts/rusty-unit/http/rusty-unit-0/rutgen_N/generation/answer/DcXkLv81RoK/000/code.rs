// Answer 0

#[test]
fn test_authority_with_port() {
    struct Authority {
        authority: &'static str,
    }

    impl Authority {
        fn parse(authority: &str) -> Result<Self, &'static str> {
            Ok(Authority { authority })
        }

        fn port(&self) -> Option<Port<&str>> {
            let bytes = self.authority;
            bytes
                .rfind(':')
                .and_then(|i| Port::from_str(&bytes[i + 1..]).ok())
        }

        fn as_str(&self) -> &str {
            self.authority
        }
    }

    struct Port<'a>(&'a str);

    impl<'a> Port<'a> {
        fn from_str(s: &'a str) -> Result<Self, &'static str> {
            if s.is_empty() {
                return Err("Port is empty");
            }
            Ok(Port(s))
        }

        fn as_u16(&self) -> u16 {
            self.0.parse().unwrap()
        }

        fn as_str(&self) -> &str {
            self.0
        }
    }

    let authority = Authority::parse("example.org:80").unwrap();
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 80);
    assert_eq!(port.as_str(), "80");
}

#[test]
fn test_authority_without_port() {
    struct Authority {
        authority: &'static str,
    }

    impl Authority {
        fn parse(authority: &str) -> Result<Self, &'static str> {
            Ok(Authority { authority })
        }

        fn port(&self) -> Option<Port<&str>> {
            let bytes = self.authority;
            bytes
                .rfind(':')
                .and_then(|i| Port::from_str(&bytes[i + 1..]).ok())
        }

        fn as_str(&self) -> &str {
            self.authority
        }
    }

    struct Port<'a>(&'a str);

    impl<'a> Port<'a> {
        fn from_str(s: &'a str) -> Result<Self, &'static str> {
            if s.is_empty() {
                return Err("Port is empty");
            }
            Ok(Port(s))
        }
    }

    let authority = Authority::parse("example.org").unwrap();
    assert!(authority.port().is_none());
}

