// Answer 0

#[test]
fn test_authority_empty_authority() {
    #[derive(Clone)]
    struct Authority {
        data: ByteStr,
    }

    #[derive(Clone)]
    struct Uri {
        authority: Authority,
    }

    #[derive(Clone)]
    struct ByteStr {
        bytes: Bytes,
    }

    impl Uri {
        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.bytes.len() == 0 {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let empty_bytes = Bytes::from_static(b"");
    let empty_authority = Authority { data: ByteStr { bytes: empty_bytes } };
    let uri = Uri { authority: empty_authority };

    assert!(uri.authority().is_none());
}

#[test]
fn test_authority_non_empty_authority() {
    #[derive(Clone)]
    struct Authority {
        data: ByteStr,
    }

    #[derive(Clone)]
    struct Uri {
        authority: Authority,
    }

    #[derive(Clone)]
    struct ByteStr {
        bytes: Bytes,
    }

    impl Uri {
        pub fn authority(&self) -> Option<&Authority> {
            if self.authority.data.bytes.len() == 0 {
                None
            } else {
                Some(&self.authority)
            }
        }
    }

    let non_empty_bytes = Bytes::from_static(b"example.com:80");
    let non_empty_authority = Authority { data: ByteStr { bytes: non_empty_bytes } };
    let uri = Uri { authority: non_empty_authority };

    assert!(uri.authority().is_some());
}

