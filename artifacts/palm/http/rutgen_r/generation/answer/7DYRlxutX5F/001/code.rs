// Answer 0

#[test]
fn test_authority_port_as_str_valid() {
    struct Authority {
        repr: String,
    }

    impl Authority {
        fn port(&self) -> Option<&Self> {
            if self.repr.contains(':') {
                Some(self)
            } else {
                None
            }
        }

        fn as_str(&self) -> &str {
            self.repr.as_ref()
        }
    }

    let authority: Authority = Authority { repr: String::from("example.org:80") };
    
    let port = authority.port().unwrap();
    assert_eq!(port.as_str(), "example.org:80");
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_authority_port_as_str_no_colon() {
    struct Authority {
        repr: String,
    }

    impl Authority {
        fn port(&self) -> Option<&Self> {
            if self.repr.contains(':') {
                Some(self)
            } else {
                None
            }
        }

        fn as_str(&self) -> &str {
            self.repr.as_ref()
        }
    }

    let authority: Authority = Authority { repr: String::from("example.org") };
    
    // This will panic because port() returns None
    let _port = authority.port().unwrap();
}

#[test]
fn test_authority_port_as_str_another_valid() {
    struct Authority {
        repr: String,
    }

    impl Authority {
        fn port(&self) -> Option<&Self> {
            if self.repr.contains(':') {
                Some(self)
            } else {
                None
            }
        }

        fn as_str(&self) -> &str {
            self.repr.as_ref()
        }
    }

    let authority: Authority = Authority { repr: String::from("example.org:443") };
    
    let port = authority.port().unwrap();
    assert_eq!(port.as_str(), "example.org:443");
}

