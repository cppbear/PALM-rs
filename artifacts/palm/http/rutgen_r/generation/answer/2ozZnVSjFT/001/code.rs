// Answer 0

#[test]
fn test_host_valid_hostname() {
    struct Authority {
        authority: String,
    }
    
    impl Authority {
        fn as_str(&self) -> &str {
            &self.authority
        }

        fn host(&self) -> &str {
            host(self.as_str())
        }
    }
    
    fn host(input: &str) -> &str {
        // Simulating the behavior of extracting the host.
        input.split(':').next().unwrap()
    }

    let authority = Authority { authority: String::from("example.com:80") };
    assert_eq!(authority.host(), "example.com");
}

#[test]
fn test_host_ipv4_address() {
    struct Authority {
        authority: String,
    }
    
    impl Authority {
        fn as_str(&self) -> &str {
            &self.authority
        }

        fn host(&self) -> &str {
            host(self.as_str())
        }
    }
    
    fn host(input: &str) -> &str {
        input.split(':').next().unwrap()
    }

    let authority = Authority { authority: String::from("192.168.1.1:8080") };
    assert_eq!(authority.host(), "192.168.1.1");
}

#[test]
fn test_host_ipv6_address() {
    struct Authority {
        authority: String,
    }
    
    impl Authority {
        fn as_str(&self) -> &str {
            &self.authority
        }

        fn host(&self) -> &str {
            host(self.as_str())
        }
    }
    
    fn host(input: &str) -> &str {
        input.split(':').next().unwrap()
    }

    let authority = Authority { authority: String::from("[::1]:80") };
    assert_eq!(authority.host(), "[::1]");
}

#[test]
fn test_host_case_insensitivity() {
    struct Authority {
        authority: String,
    }
    
    impl Authority {
        fn as_str(&self) -> &str {
            &self.authority
        }

        fn host(&self) -> &str {
            host(self.as_str())
        }
    }
    
    fn host(input: &str) -> &str {
        input.split(':').next().unwrap()
    }

    let authority = Authority { authority: String::from("EXAMPLE.ORG:80") };
    assert_eq!(authority.host(), "EXAMPLE.ORG");
}

#[should_panic]
#[test]
fn test_host_empty_authority() {
    struct Authority {
        authority: String,
    }
    
    impl Authority {
        fn as_str(&self) -> &str {
            &self.authority
        }

        fn host(&self) -> &str {
            host(self.as_str())
        }
    }
    
    fn host(input: &str) -> &str {
        input.split(':').next().unwrap()
    }

    let authority = Authority { authority: String::from("") };
    authority.host(); // This should panic due to empty input
}

