// Answer 0

#[test]
fn test_authority_non_empty() {
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"example.com:80"),
        },
    };
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority,
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/path"),
            },
            query: 0,
        },
    };
    uri.authority();
}

#[test]
fn test_authority_with_username_and_password() {
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"username:password@example.com:80"),
        },
    };
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority,
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/path"),
            },
            query: 0,
        },
    };
    uri.authority();
}

#[test]
fn test_authority_case_insensitivity() {
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"ExAmPlE.cOm:80"),
        },
    };
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority,
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/path"),
            },
            query: 0,
        },
    };
    uri.authority();
}

#[test]
fn test_authority_long_data() {
    let long_authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"x".repeat(255).as_bytes()),
        },
    };
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: long_authority,
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/path"),
            },
            query: 0,
        },
    };
    uri.authority();
}

