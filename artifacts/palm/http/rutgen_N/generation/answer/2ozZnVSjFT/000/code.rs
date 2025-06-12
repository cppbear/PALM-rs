// Answer 0

#[test]
fn test_host_with_registered_name() {
    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
        
        fn host(&self) -> &str {
            host(self.as_str())
        }
    }

    fn host(authority: &str) -> &str {
        let parts: Vec<&str> = authority.split(':').collect();
        parts[0] // The host is the part before the colon
    }

    let authority = Authority {
        value: String::from("example.org:80"),
    };

    assert_eq!(authority.host(), "example.org");
}

#[test]
fn test_host_with_ipv4() {
    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
        
        fn host(&self) -> &str {
            host(self.as_str())
        }
    }

    fn host(authority: &str) -> &str {
        let parts: Vec<&str> = authority.split(':').collect();
        parts[0] // The host is the part before the colon
    }

    let authority = Authority {
        value: String::from("192.168.1.1:8080"),
    };

    assert_eq!(authority.host(), "192.168.1.1");
}

#[test]
fn test_host_with_ip_literal() {
    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
        
        fn host(&self) -> &str {
            host(self.as_str())
        }
    }

    fn host(authority: &str) -> &str {
        let parts: Vec<&str> = authority.split(':').collect();
        parts[0] // The host is the part before the colon
    }

    let authority = Authority {
        value: String::from("[::1]:8080"),
    };

    assert_eq!(authority.host(), "[::1]");
}

#[test]
fn test_host_with_case_insensitivity() {
    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
        
        fn host(&self) -> &str {
            host(self.as_str())
        }
    }

    fn host(authority: &str) -> &str {
        let parts: Vec<&str> = authority.split(':').collect();
        parts[0] // The host is the part before the colon
    }

    let authority1 = Authority {
        value: String::from("Example.Com:80"),
    };
    let authority2 = Authority {
        value: String::from("example.com:80"),
    };

    assert_eq!(authority1.host().to_lowercase(), authority2.host().to_lowercase());
}

#[test]
fn test_host_with_empty_authority() {
    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
        
        fn host(&self) -> &str {
            host(self.as_str())
        }
    }

    fn host(authority: &str) -> &str {
        if authority.is_empty() {
            ""
        } else {
            let parts: Vec<&str> = authority.split(':').collect();
            parts[0] // The host is the part before the colon
        }
    }

    let authority = Authority {
        value: String::new(),
    };

    assert_eq!(authority.host(), "");
}

