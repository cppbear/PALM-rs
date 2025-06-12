// Answer 0

#[test]
fn test_authority_empty_data() {
    struct Authority {
        data: String,
    }

    struct Uri {
        authority: Authority,
    }

    impl Uri {
        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let uri = Uri {
        authority: Authority { data: String::new() },
    };

    assert!(uri.authority().is_none());
}

#[test]
fn test_authority_non_empty_data() {
    struct Authority {
        data: String,
    }

    struct Uri {
        authority: Authority,
    }

    impl Uri {
        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let uri = Uri {
        authority: Authority { data: String::from("example.com:80") },
    };

    assert_eq!(uri.authority().map(|a| a.data.as_str()), Some("example.com:80"));
}

