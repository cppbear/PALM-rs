// Answer 0

#[test]
fn test_port_as_str() {
    struct DummyStr {
        value: String,
    }

    impl AsRef<str> for DummyStr {
        fn as_ref(&self) -> &str {
            &self.value
        }
    }

    let dummy_repr = DummyStr { value: String::from("80") };
    let port = Port { port: 80, repr: dummy_repr };

    assert_eq!(port.as_str(), "80");
}

#[test]
fn test_port_creation_from_str() {
    struct DummyStr {
        value: String,
    }

    impl AsRef<str> for DummyStr {
        fn as_ref(&self) -> &str {
            &self.value
        }
    }

    let valid_str = DummyStr { value: String::from("8080") };
    let port_result = Port::from_str(valid_str);
    
    assert!(port_result.is_ok());
    let port = port_result.unwrap();
    assert_eq!(port.port, 8080);
    assert_eq!(port.as_str(), "8080");
}

#[test]
fn test_port_creation_invalid_str() {
    struct DummyStr {
        value: String,
    }

    impl AsRef<str> for DummyStr {
        fn as_ref(&self) -> &str {
            &self.value
        }
    }

    let invalid_str = DummyStr { value: String::from("invalid") };
    let port_result = Port::from_str(invalid_str);
    
    assert!(port_result.is_err());
}

