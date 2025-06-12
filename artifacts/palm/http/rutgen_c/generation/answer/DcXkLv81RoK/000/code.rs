// Answer 0

#[test]
fn test_port_with_valid_authority() {
    struct PortStr(u16);
    
    impl FromStr for PortStr {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let port = s.parse::<u16>()?;
            Ok(PortStr(port))
        }
    }

    let authority = Authority::from_static("example.org:80");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 80);
    assert_eq!(port.as_str(), "80");
}

#[test]
fn test_port_without_valid_authority() {
    let authority = Authority::from_static("example.org");
    assert!(authority.port().is_none());
}

#[test]
fn test_port_with_edge_case() {
    let authority = Authority::from_static("localhost:65535");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 65535);
    assert_eq!(port.as_str(), "65535");
}

#[test]
fn test_port_with_invalid_port() {
    struct InvalidPortStr;

    impl FromStr for InvalidPortStr {
        type Err = std::num::ParseIntError;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            Err(std::num::ParseIntError::from(std::num::IntErrorKind::InvalidDigit))
        }
    }

    let authority = Authority::from_static("example.org:invalid");
    assert!(authority.port().is_none());
}

