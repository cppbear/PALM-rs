// Answer 0

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

    let authority = Authority {
        data: String::from("example.com:123"),
    };

    let uri = Uri { authority };

    assert_eq!(uri.authority().map(|a| a.data.as_str()), Some("example.com:123"));
}

#[test]
fn test_authority_with_username_and_password() {
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

    let authority = Authority {
        data: String::from("username:password@example.com:123"),
    };

    let uri = Uri { authority };

    assert_eq!(uri.authority().map(|a| a.data.as_str()), Some("username:password@example.com:123"));
}

