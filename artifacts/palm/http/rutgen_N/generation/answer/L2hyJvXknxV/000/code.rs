// Answer 0

#[test]
fn test_authority_absolute_uri() {
    struct Uri {
        authority: Authority,
    }

    struct Authority {
        data: String,
    }

    impl Uri {
        fn parse(input: &str) -> Result<Self, &'static str> {
            if input.contains("://") {
                let authority_part = input.split("://").nth(1).unwrap().split('/').next().unwrap();
                return Ok(Uri {
                    authority: Authority {
                        data: authority_part.to_string(),
                    },
                });
            }
            Err("Invalid URI")
        }
        
        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let uri: Uri = Uri::parse("http://example.org:80/hello/world").unwrap();
    assert_eq!(uri.authority().map(|a| a.data.as_str()), Some("example.org:80"));
}

#[test]
fn test_authority_relative_uri() {
    struct Uri {
        authority: Authority,
    }

    struct Authority {
        data: String,
    }

    impl Uri {
        fn parse(input: &str) -> Result<Self, &'static str> {
            if input.contains("://") {
                let authority_part = input.split("://").nth(1).unwrap().split('/').next().unwrap();
                return Ok(Uri {
                    authority: Authority {
                        data: authority_part.to_string(),
                    },
                });
            }
            return Ok(Uri {
                authority: Authority {
                    data: String::new(),
                },
            });
        }

        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let uri: Uri = Uri::parse("/hello/world").unwrap();
    assert!(uri.authority().is_none());
}

