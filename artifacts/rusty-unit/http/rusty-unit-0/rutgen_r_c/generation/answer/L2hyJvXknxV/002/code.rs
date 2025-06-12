// Answer 0

#[test]
fn test_authority_non_empty() {
    // Helper struct to create necessary instances for testing
    struct TestAuthority {
        data: ByteStr,
    }

    impl TestAuthority {
        fn new(data: &[u8]) -> Self {
            TestAuthority {
                data: ByteStr {
                    bytes: Bytes::copy_from_slice(data),
                },
            }
        }
    }
    
    struct TestUri {
        authority: Authority,
    }
    
    impl TestUri {
        fn new(authority_data: &[u8]) -> Self {
            TestUri {
                authority: Authority {
                    data: ByteStr {
                        bytes: Bytes::copy_from_slice(authority_data),
                    },
                },
            }
        }

        fn authority(&self) -> Option<&Authority> {
            if self.authority.data.bytes.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    // Test case where authority data is non-empty
    let uri = TestUri::new(b"example.com:80");
    assert!(uri.authority().is_some());
    assert_eq!(uri.authority().map(|a| &a.data), Some(&ByteStr { bytes: Bytes::copy_from_slice(b"example.com:80") }));
}

#[test]
fn test_authority_empty() {
    // Using the same structure but with empty authority
    struct TestEmptyUri {
        authority: Authority,
    }

    impl TestEmptyUri {
        fn new() -> Self {
            TestEmptyUri {
                authority: Authority {
                    data: ByteStr {
                        bytes: Bytes::new(),
                    },
                },
            }
        }

        fn authority(&self) -> Option<&Authority> {
            if self.authority.data.bytes.is_empty() {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let uri = TestEmptyUri::new();
    assert!(uri.authority().is_none());
}

